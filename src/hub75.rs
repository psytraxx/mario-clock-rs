// Inspired by
// - https://github.com/polyfloyd/ledcat/blob/master/src/device/hub75.rs
// - https://github.com/mmou/led-marquee/blob/8c88531a6938edff6db829ca21c15304515874ea/src/hub.rs
// - https://github.com/adafruit/RGB-matrix-Panel/blob/master/RGBmatrixPanel.cpp
// - https://www.mikrocontroller.net/topic/452187 (sorry, german only)
// - https://github.com/mrcodetastic/ESP32-HUB75-MatrixPanel-DMA/issues/433
// - https://github.com/mrcodetastic/ESP32-HUB75-MatrixPanel-DMA
// - https://github.com/ironsheep/P2-HUB75-LED-Matrix-Driver
// - https://github.com/TillFleisch/ESPHome-HUB75-MatrixDisplayWrapper/blob/main/components/hub75_matrix_display/matrix_display.cpp

use embedded_hal::{delay::DelayNs, digital::OutputPin};

/// # Theory of Operation
/// This display is essentially split in half, with the top 16 rows being
/// controlled by one set of shift registers (r1, g1, b1) and the botton 16
/// rows by another set (r2, g2, b2). So, the best way to update it is to
/// show one of the botton and top rows in tandem. The row (between 0-15) is then
/// selected by the A, B, C, D pins, which are just, as one might expect, the bits 0 to 3.
/// Pin F is used by the 64x64 display to get 5 bit row addressing (1/32 row scan rate)
///
/// The display doesn't really do brightness, so we have to do it ourselves, by
/// rendering the same frame multiple times, with some pixels being turned of if
/// they are darker (pwm)
const ADDRESSABLE_ROWS: usize = 32; // Number of rows addressable with A/B/C/D/F (0-31)

// This table remaps linear input values
// (the numbers we’d like to use; e.g. 127 = half brightness)
// to nonlinear gamma-corrected output values
// (numbers producing the desired effect on the LED;
// e.g. 36 = half brightness).
const GAMMA8: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5,
    5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14,
    14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 24, 24, 25, 25, 26, 27,
    27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36, 37, 38, 39, 39, 40, 41, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 50, 51, 52, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 66, 67, 68, 69, 70, 72,
    73, 74, 75, 77, 78, 79, 81, 82, 83, 85, 86, 87, 89, 90, 92, 93, 95, 96, 98, 99, 101, 102, 104,
    105, 107, 109, 110, 112, 114, 115, 117, 119, 120, 122, 124, 126, 127, 129, 131, 133, 135, 137,
    138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 167, 169, 171, 173, 175,
    177, 180, 182, 184, 186, 189, 191, 193, 196, 198, 200, 203, 205, 208, 210, 213, 215, 218, 220,
    223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 249, 252, 255,
];

// Represents the color data for one pixel in the top half (r1,g1,b1) and one in the bottom half (r2,g2,b2)
type PixelData = (u8, u8, u8, u8, u8, u8);
type RowData = [PixelData; 64]; // Data for 64 columns

/// Driver for HUB75-compatible RGB LED matrix panels.
///
/// This driver manages a framebuffer and uses Binary Code Modulation (BCM), a form of PWM,
/// to control the brightness of individual LEDs. It assumes a panel width of 64 pixels
/// and uses `ADDRESSABLE_ROWS` (typically 16 or 32) for row selection.
/// For panels with 32 addressable rows (like 64x64 panels with 1/32 scan rate),
/// it drives the top and bottom halves concurrently using the R1/G1/B1 and R2/G2/B2 pins.
///
/// It implements `embedded_graphics::prelude::DrawTarget` to allow drawing graphics primitives.
pub struct Hub75<PINS>
where
    PINS: Outputs,
{
    /// Internal framebuffer storing gamma-corrected color data.
    /// `data[row_addr][col]` holds the `PixelData` for the pixel at `(col, row_addr)` in the top half
    /// and `(col, row_addr + ADDRESSABLE_ROWS)` in the bottom half.
    data: [RowData; ADDRESSABLE_ROWS],
    /// The step value used in the BCM brightness control loop. Determined by `brightness_bits`.
    brightness_step: u8,
    /// The number of brightness levels minus one. Determined by `brightness_bits`.
    brightness_count: u8,
    /// The GPIO pins used to control the panel.
    pins: PINS,
}

/// A trait abstracting the output pins required by the HUB75 panel.
/// Implemented for a tuple `(r1, g1, b1, r2, g2, b2, a, b, c, d, f, clk, lat, oe)`
/// Implemented for a tuple `(r1, g1, b1, r2, g2, b2, a, b, c, d, clk, lat, oe)`
/// with every element implementing `OutputPin`
/// f pin is needed for 64x64 matrix support
pub trait Outputs {
    type Error;
    type R1: OutputPin<Error = Self::Error>;
    type G1: OutputPin<Error = Self::Error>;
    type B1: OutputPin<Error = Self::Error>;
    type R2: OutputPin<Error = Self::Error>;
    type G2: OutputPin<Error = Self::Error>;
    type B2: OutputPin<Error = Self::Error>;
    type A: OutputPin<Error = Self::Error>;
    type B: OutputPin<Error = Self::Error>;
    type C: OutputPin<Error = Self::Error>;
    type D: OutputPin<Error = Self::Error>;
    type F: OutputPin<Error = Self::Error>;
    type CLK: OutputPin<Error = Self::Error>;
    type LAT: OutputPin<Error = Self::Error>;
    type OE: OutputPin<Error = Self::Error>;
    fn r1(&mut self) -> &mut Self::R1;
    fn g1(&mut self) -> &mut Self::G1;
    fn b1(&mut self) -> &mut Self::B1;
    fn r2(&mut self) -> &mut Self::R2;
    fn g2(&mut self) -> &mut Self::G2;
    fn b2(&mut self) -> &mut Self::B2;
    fn a(&mut self) -> &mut Self::A;
    fn b(&mut self) -> &mut Self::B;
    fn c(&mut self) -> &mut Self::C;
    fn d(&mut self) -> &mut Self::D;
    fn f(&mut self) -> &mut Self::F;
    fn clk(&mut self) -> &mut Self::CLK;
    fn lat(&mut self) -> &mut Self::LAT;
    fn oe(&mut self) -> &mut Self::OE;
}

impl<
        E,
        R1: OutputPin<Error = E>,
        G1: OutputPin<Error = E>,
        B1: OutputPin<Error = E>,
        R2: OutputPin<Error = E>,
        G2: OutputPin<Error = E>,
        B2: OutputPin<Error = E>,
        A: OutputPin<Error = E>,
        B: OutputPin<Error = E>,
        C: OutputPin<Error = E>,
        D: OutputPin<Error = E>,
        F: OutputPin<Error = E>,
        CLK: OutputPin<Error = E>,
        LAT: OutputPin<Error = E>,
        OE: OutputPin<Error = E>,
    > Outputs for (R1, G1, B1, R2, G2, B2, A, B, C, D, F, CLK, LAT, OE)
{
    type Error = E;
    type R1 = R1;
    type G1 = G1;
    type B1 = B1;
    type R2 = R2;
    type G2 = G2;
    type B2 = B2;
    type A = A;
    type B = B;
    type C = C;
    type D = D;
    type F = F;
    type CLK = CLK;
    type LAT = LAT;
    type OE = OE;
    fn r1(&mut self) -> &mut R1 {
        &mut self.0
    }
    fn g1(&mut self) -> &mut G1 {
        &mut self.1
    }
    fn b1(&mut self) -> &mut B1 {
        &mut self.2
    }
    fn r2(&mut self) -> &mut R2 {
        &mut self.3
    }
    fn g2(&mut self) -> &mut G2 {
        &mut self.4
    }
    fn b2(&mut self) -> &mut B2 {
        &mut self.5
    }
    fn a(&mut self) -> &mut A {
        &mut self.6
    }
    fn b(&mut self) -> &mut B {
        &mut self.7
    }
    fn c(&mut self) -> &mut C {
        &mut self.8
    }
    fn d(&mut self) -> &mut D {
        &mut self.9
    }
    fn f(&mut self) -> &mut F {
        &mut self.10
    }
    fn clk(&mut self) -> &mut CLK {
        &mut self.11
    }
    fn lat(&mut self) -> &mut LAT {
        &mut self.12
    }
    fn oe(&mut self) -> &mut OE {
        &mut self.13
    }
}

impl<PINS: Outputs> Hub75<PINS> {
    /// Creates a new `Hub75` driver instance.
    ///
    /// # Arguments
    ///
    /// * `pins`: An instance of a type implementing the `Outputs` trait, providing access
    ///   to the necessary GPIO pins. A tuple of `OutputPin` implementors is the common way
    ///   to provide this.
    /// * `brightness_bits`: The number of bits to use for brightness control (1-8).
    ///   Higher values provide more color depth but increase the time required for
    ///   each refresh cycle in the `output` method, potentially causing flicker if
    ///   the refresh rate becomes too low. 3 or 4 bits are often a good balance.
    ///
    /// # Panics
    ///
    /// Panics if `brightness_bits` is not between 1 and 8 (inclusive).
    pub fn new(pins: PINS, brightness_bits: u8) -> Self {
        assert!(
            brightness_bits < 9 && brightness_bits > 0,
            "Brightness bits must be between 1 and 8"
        );
        // Initialize framebuffer with all pixels off
        let data = [[(0, 0, 0, 0, 0, 0); 64]; ADDRESSABLE_ROWS];
        // Calculate PWM parameters based on desired brightness bits
        let brightness_step = 1 << (8 - brightness_bits);
        let brightness_count = ((1 << brightness_bits as u16) - 1) as u8;
        Self {
            data,
            brightness_step,
            brightness_count,
            pins,
        }
    }

    /// Clocks in the pixel data for a single row pair based on the brightness threshold.
    fn clock_in_row_data(
        &mut self,
        row_index: usize, // Changed parameter
        brightness_threshold: u8,
    ) -> Result<(), PINS::Error> {
        let row_data = &self.data[row_index]; // Borrow self.data immutably *inside* the function
        for element in row_data.iter() {
            // Set R1, G1, B1
            self.pins
                .r1()
                .set_state((element.0 >= brightness_threshold).into())?;
            self.pins
                .g1()
                .set_state((element.1 >= brightness_threshold).into())?;
            self.pins
                .b1()
                .set_state((element.2 >= brightness_threshold).into())?;
            // Set R2, G2, B2
            self.pins
                .r2()
                .set_state((element.3 >= brightness_threshold).into())?;
            self.pins
                .g2()
                .set_state((element.4 >= brightness_threshold).into())?;
            self.pins
                .b2()
                .set_state((element.5 >= brightness_threshold).into())?;

            // Pulse clock
            self.pins.clk().set_high()?;
            self.pins.clk().set_low()?;
        }
        Ok(())
    }

    /// Sets the row address pins (A, B, C, D, F).
    fn select_row(&mut self, row_index: usize) -> Result<(), PINS::Error> {
        self.pins.a().set_state(((row_index & 1) != 0).into())?;
        self.pins.b().set_state(((row_index & 2) != 0).into())?;
        self.pins.c().set_state(((row_index & 4) != 0).into())?;
        self.pins.d().set_state(((row_index & 8) != 0).into())?;
        self.pins.f().set_state(((row_index & 16) != 0).into())?;
        Ok(())
    }

    /// Pulses the latch pin to load the row data.
    fn latch_data<DELAY: DelayNs>(&mut self, delay: &mut DELAY) -> Result<(), PINS::Error> {
        // Prevents ghosting, no idea why
        delay.delay_us(2);
        self.pins.lat().set_low()?;
        delay.delay_us(2);
        self.pins.lat().set_high()?;
        Ok(())
    }

    /// Refreshes the display panel with the current framebuffer content.
    ///
    /// This method implements Binary Code Modulation (BCM) for brightness control.
    /// It iterates through `brightness_count` levels. In each level, it iterates
    /// through all `ADDRESSABLE_ROWS`. For each row pair, it clocks out the pixel data,
    /// comparing the stored gamma-corrected color value against the current brightness
    /// threshold. It then latches the data, selects the row address, and enables the output.
    ///
    /// **This method must be called frequently and regularly in a loop** (e.g., from a timer
    /// interrupt or a dedicated thread) to maintain a stable, flicker-free image on the panel.
    /// The required frequency depends on `brightness_bits` and the panel characteristics,
    /// but is typically in the range of hundreds of Hz or more.
    ///
    /// # Arguments
    ///
    /// * `delay`: A `DelayNs` provider used for short delays required by the panel timing.
    pub fn output<DELAY: DelayNs>(&mut self, delay: &mut DELAY) -> Result<(), PINS::Error> {
        // Enable the output for the previous row (from the last call or last row of previous cycle)
        self.pins.oe().set_low()?;

        // PWM cycle
        for mut brightness in 0..self.brightness_count {
            // Map the loop counter (0..brightness_count) to the actual brightness threshold
            // used for comparison against pixel data.
            brightness = (brightness + 1).saturating_mul(self.brightness_step);

            // Iterate through all addressable rows (0-31)
            for row_index in 0..ADDRESSABLE_ROWS {
                // --- Shift out pixel data for this row pair ---
                // For the current brightness level, clock out the data for all 64 columns
                // for the top half (r1,g1,b1) and bottom half (r2,g2,b2) simultaneously.
                // A pixel is turned on if its stored gamma-corrected value >= current brightness threshold.
                self.clock_in_row_data(row_index, brightness)?;

                // --- Latch and select next row ---

                // Disable output while changing row and latching
                self.pins.oe().set_high()?;

                // Latch the data shifted into the registers.
                self.latch_data(delay)?;

                // Set the address pins (A-F) to select the physical row to display the latched data.
                self.select_row(row_index)?;

                // Small delay required by some panels before enabling output.
                delay.delay_us(2);

                // Re-enable output for the newly selected row
                self.pins.oe().set_low()?;
            }
        }
        // Disable the output at the end of the full refresh cycle
        // Prevents one row from being much brighter than the others
        self.pins.oe().set_high()?;
        Ok(())
    }

    /// Clears the entire display framebuffer (sets all pixels to black).
    ///
    /// This is generally faster than using `draw_iter` to fill the display
    /// with black pixels.
    ///
    /// It's a bit faster than using the embedded_graphics interface
    /// to do the same
    pub fn clear(&mut self) {
        for row in self.data.iter_mut() {
            for e in row.iter_mut() {
                e.0 = 0;
                e.1 = 0;
                e.2 = 0;
                e.3 = 0;
                e.4 = 0;
                e.5 = 0;
            }
        }
    }
}

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{Dimensions, DrawTarget, Point, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};
impl<PINS: Outputs> DrawTarget for Hub75<PINS> {
    type Error = PINS::Error;
    type Color = Rgb888;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            // Map the y-coordinate to the row address (0-31)
            let row_addr = coord[1] % ADDRESSABLE_ROWS as i32;
            // Ensure coordinates are within the 64x64 bounds reported by Dimensions
            if coord[0] >= 0 && coord[0] < 64 && row_addr >= 0 && coord[1] < 64 {
                let col = coord[0] as usize;
                let row_idx = row_addr as usize;
                let data = &mut self.data[row_idx][col];

                // Apply gamma correction and store in the correct half of the PixelData tuple
                if coord[1] >= ADDRESSABLE_ROWS as i32 {
                    // Bottom half (R2, G2, B2)
                    data.3 = GAMMA8[color.r() as usize];
                    data.4 = GAMMA8[color.g() as usize];
                    data.5 = GAMMA8[color.b() as usize];
                } else {
                    // Top half (R1, G1, B1)
                    data.0 = GAMMA8[color.r() as usize];
                    data.1 = GAMMA8[color.g() as usize];
                    data.2 = GAMMA8[color.b() as usize];
                }
            }
        }
        Ok(())
    }
}

impl<PINS: Outputs> Dimensions for Hub75<PINS> {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        // This driver assumes a 64-pixel wide display.
        // It uses ADDRESSABLE_ROWS (32) for row addressing via A-F pins.
        // The draw_iter logic maps coordinates >= 32 to the second set of color pins (r2,g2,b2),
        // effectively treating the display as 64x64 composed of two 64x32 halves.
        Rectangle::new(Point::zero(), Size::new(64, 64))
    }
}
