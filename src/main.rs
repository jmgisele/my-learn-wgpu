use pollster::block_on;
use wgpu_1::run;
fn main() {
    pollster::block_on(run());
}
