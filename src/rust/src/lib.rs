use extendr_api::prelude::*;
use rapier2d::na;
use rapier2d::{parry::partitioning::IndexedData, prelude::*};

mod result;

pub struct Rapier2DWorld {
    cur_step: usize,
    physics_pipeline: PhysicsPipeline,
    gravity: rapier2d::na::Vector2<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    body_handles: Vec<RigidBodyHandle>,
    colliders: ColliderSet,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    // physics_hooks: dyn PhysicsHooks,
    // event_handler: dyn EventHandler,
}

/// The 2D World of Rapier
///
/// @export
#[extendr(use_try_from = true)]
impl Rapier2DWorld {
    const GRAVITY: na::Vector2<f32> = vector![0.0, -9.81];

    pub fn new() -> Self {
        Self {
            cur_step: 0,
            physics_pipeline: PhysicsPipeline::new(),
            gravity: Self::GRAVITY,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            body_handles: Vec::new(),
            colliders: ColliderSet::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    pub fn step(&mut self, n: i32) -> Robj {
        let n = n as usize;
        let mut frame = extendr_api::Integers::new(n * self.object_count());
        let mut index = extendr_api::Integers::new(n * self.object_count());
        let mut x = extendr_api::Doubles::new(n * self.object_count());
        let mut y = extendr_api::Doubles::new(n * self.object_count());
        let mut size = extendr_api::Doubles::new(n * self.object_count());
        let mut angle = extendr_api::Doubles::new(n * self.object_count());

        let mut i = 0_usize;

        for _ in 0..n {
            self.physics_pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.bodies,
                &mut self.colliders,
                &mut self.impulse_joints,
                &mut self.multibody_joints,
                &mut self.ccd_solver,
                Some(&mut self.query_pipeline),
                &(),
                &(),
            );

            for &h in &self.body_handles {
                let body = &self.bodies.get(h).unwrap();
                frame.set_elt(i, (self.cur_step as i32).into());
                index.set_elt(i, (h.index() as i32).into());
                x.set_elt(i, (body.translation().x as f64).into());
                y.set_elt(i, (body.translation().y as f64).into());
                angle.set_elt(i, (body.rotation().angle() as f64).into());

                let b = self.bodies.get(h).unwrap();
                let c = self.colliders.get(b.colliders()[0]).unwrap();
                let len = match c.shape().as_typed_shape() {
                    TypedShape::Ball(ball) => ball.radius,
                    TypedShape::Cuboid(cuboid) => cuboid.half_extents.x * 2.,
                    _ => unreachable!("Unknown shape!"),
                };
                size.set_elt(i, (len as f64).into());

                i += 1;
            }

            self.cur_step += 1;
        }

        result::ResultTibble {
            frame,
            index,
            x,
            y,
            size,
            angle,
        }
        .try_into()
        .unwrap()
    }

    pub fn add_ball(&mut self, x: f32, y: f32, radius: f32, restitution: f32) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .ccd_enabled(true)
            .build();
        let collider = ColliderBuilder::ball(radius)
            .restitution(restitution)
            .restitution_combine_rule(CoefficientCombineRule::Multiply)
            .build();
        let ball_body_handle = self.bodies.insert(rigid_body);
        self.colliders
            .insert_with_parent(collider, ball_body_handle, &mut self.bodies);
        self.body_handles.push(ball_body_handle);
    }

    pub fn add_line(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, restitution: f32) {
        let p0 = point![x0, y0];
        let p1 = point![x1, y1];
        let v = p1 - p0;
        let length = v.norm();
        let angle = v.y.atan2(v.x);

        let rigid_body = RigidBodyBuilder::dynamic()
            .rotation(angle)
            .translation(p0.coords + 0.5 * v)
            .ccd_enabled(true)
            .build();

        let collider = ColliderBuilder::cuboid(length / 2., 0.01)
            .restitution(restitution)
            .restitution_combine_rule(CoefficientCombineRule::Multiply)
            .build();

        let line_body_handle = self.bodies.insert(rigid_body);
        self.colliders
            .insert_with_parent(collider, line_body_handle, &mut self.bodies);
        self.body_handles.push(line_body_handle);
    }

    pub fn add_fixed_segment(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, restitution: f32) {
        let collider = ColliderBuilder::segment(point![x0, y0], point![x1, y1])
            .restitution(restitution)
            .build();
        let pos = collider.position();
        rprintln!("{:?}", pos);

        self.colliders.insert(collider);
    }

    fn object_count(&self) -> usize {
        self.body_handles
            .iter()
            .map(|h| {
                let b = self.bodies.get(*h).unwrap();
                let c = self.colliders.get(b.colliders()[0]).unwrap();
                // TODO
                match c.shape().as_typed_shape() {
                    TypedShape::Ball(_) => 1,
                    TypedShape::Cuboid(_) => 1,
                    _ => unreachable!("Unknown shape!"),
                }
            })
            .sum::<usize>()
    }
}

impl Default for Rapier2DWorld {
    fn default() -> Self {
        Self::new()
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rapierr;
    impl Rapier2DWorld;
}
