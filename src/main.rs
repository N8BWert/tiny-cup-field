extern crate ncomm;

use ncomm::executor::{simple_multi_executor::SimpleMultiExecutor, Executor};

pub mod camera;
pub mod referee;

use camera::camera_node::CameraNode;
use referee::referee_node::RefereeNode;

fn main() {
    let mut referee = RefereeNode::new("referee", 20);
    let mut camera = CameraNode::new("camera", (1000 / 30) as u128);
    let mut executor = SimpleMultiExecutor::new_with(
        vec![
            ("Referee Thread", &mut referee),
            ("Camera Thread", &mut camera)
        ]
    );

    // Start Executor
    executor.start();

    // Loop The Executor
    executor.update_loop();
}
