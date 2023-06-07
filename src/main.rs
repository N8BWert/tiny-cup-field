extern crate ncomm;
extern crate tiny_cup_ref;

use ncomm::executor::{simple_multi_executor::SimpleMultiExecutor, Executor};

pub mod camera;

use camera::camera_node::CameraNode;

use tiny_cup_ref::referee_node::RefereeNode;

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
