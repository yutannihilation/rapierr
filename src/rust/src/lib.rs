use extendr_api::prelude::*;
use rapier2d::prelude::*;

mod result;

fn bouncing_ball_inner() -> Robj {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
    collider_set.insert(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 1.0])
        .build();
    let collider = ColliderBuilder::ball(0.05).restitution(0.85).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    const LEN: usize = 800;

    let mut frame: Vec<i32> = Vec::with_capacity(LEN);
    let mut x: Vec<f32> = Vec::with_capacity(LEN);
    let mut y: Vec<f32> = Vec::with_capacity(LEN);

    /* Run the game loop, stepping the simulation once per frame. */
    for i in 0..LEN {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set[ball_body_handle];
        frame.push(i as _);
        x.push(ball_body.translation().x);
        y.push(ball_body.translation().y);
    }

    result::ResultTibble { frame, x, y }.try_into().unwrap()
}

/// Return rapier results
/// @export
#[extendr(use_try_from = true)]
fn bouncing_ball() -> Robj {
    bouncing_ball_inner()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rpr2dr;
    fn bouncing_ball;
}
