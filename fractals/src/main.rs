use fractals::julia::julia;
use fractals::mandelbrot::mandelbrot;

fn main() {
    let width = 2560;
    let height = 1080;
    let save_image = true;
    mandelbrot("naive", width, height, save_image);

    julia(1000, 1000);
}
