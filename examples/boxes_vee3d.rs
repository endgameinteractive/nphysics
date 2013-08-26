#[link(name     = "boxes_vee3d"
       , vers   = "0.0"
       , author = "Sébastien Crozet"
       , uuid   = "4f1fb8c5-5f02-4d45-89f3-64acfd1718fa")];
#[crate_type = "bin"];
#[warn(non_camel_case_types)]

extern mod std;
extern mod extra;
extern mod nphysics;
extern mod nalgebra;
extern mod ncollide;
extern mod graphics3d;

use std::num::One;
use nalgebra::vec::Vec3;
use nalgebra::traits::translation::Translation;
use ncollide::geom::box::Box;
use ncollide::geom::plane::Plane;
use ncollide::broad::dbvt_broad_phase::DBVTBroadPhase;
use nphysics::object::body;
use nphysics::object::rigid_body::{RigidBody, Static, Dynamic};
use nphysics::object::implicit_geom::DefaultGeom;
use nphysics::world::world::World;
use nphysics::aliases::dim3;
use nphysics::integration::body_force_generator::BodyForceGenerator;
use nphysics::integration::rigid_body_integrator::RigidBodySmpEulerIntegrator;
use nphysics::integration::swept_ball_motion_clamping::SweptBallMotionClamping;
use nphysics::detection::collision::bodies_bodies::{BodiesBodies, Dispatcher};
use nphysics::detection::island_activation_manager::IslandActivationManager;
use nphysics::resolution::constraint::accumulated_impulse_solver::AccumulatedImpulseSolver;
use nphysics::resolution::constraint::contact_equation::VelocityAndPosition;
use graphics3d::engine::GraphicsManager;


fn main() {
    GraphicsManager::simulate(boxes_vee_3d)
}

pub fn boxes_vee_3d(graphics: &mut GraphicsManager)
                -> (dim3::World3d<f64>,
                    @mut dim3::DBVTCollisionDetector3d<f64>,
                    @mut dim3::DBVTSweptBallMotionClamping3d<f64>) {
    /*
     * Setup the physics world
     */
    let mut world = World::new();

    // For the intergration
    let gravity = Vec3::new(0.0, -9.81, 0.0);
    let tornado = Vec3::new(0.0, 0.0, 0.0);

    let forces: @mut dim3::ForceGenerator3d<f64> = @mut BodyForceGenerator::new(gravity, tornado);
    let integrator: @mut dim3::RigidBodyIntegrator3d<f64> = @mut RigidBodySmpEulerIntegrator::new();

    /*
     * For the collision detection
     */
    // Collision Dispatcher
    let dispatcher: dim3::Dispatcher3d<f64> = Dispatcher::new();
    // Broad phase
    let broad_phase = @mut DBVTBroadPhase::new(dispatcher, 0.08f64);

    // CCD handler
    let ccd = @mut SweptBallMotionClamping::new(broad_phase, true);

    // Collision detector
    let detector = @mut BodiesBodies::new(broad_phase, false);

    /*
     * For constraints resolution
     */
    let solver: @mut dim3::ContactSolver3d<f64> =
        @mut AccumulatedImpulseSolver::new(0.1f64, VelocityAndPosition(0.2, 0.2, 0.08), 1.0, 10, 10);

    /*
     * Add everything to the world
     */
    world.add_integrator(forces);
    world.add_integrator(integrator);
    world.add_integrator(ccd);
    world.add_detector(detector);
    world.add_solver(solver);

    // NOTE: this must be done _after_ the addition of every other controllers
    let sleep: @mut dim3::IslandActivationManager3d<f64> =
        @mut IslandActivationManager::new(1.0, 0.01, &mut world);
    world.add_detector(sleep);

    /*
     * Plane
     */
    let geom = Plane::new(Vec3::y());
    let body = @mut RigidBody::new(DefaultGeom::new_plane(geom), 0.0f64, Static, 0.3, 0.6);

    world.add_object(@mut body::RigidBody(body));
    graphics.add_plane(body, &geom);

    /*
     * Create the boxes
     */
    let num     = 8;
    let rad     = 1.0;
    let shift   = rad * 2.0;
    let centerx = shift * (num as f64) / 2.0;
    let centery = shift / 2.0;
    let centerz = shift * (num as f64) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            for k in range(0u, num) {
                let x = i as f64 * shift - centerx;
                let y = j as f64 * shift + centery;
                let z = k as f64 * shift - centerz;

                let box  = Box::new(Vec3::new(rad, rad, rad));
                let geom = DefaultGeom::new_box(box);
                let body = @mut RigidBody::new(geom, 1.0f64, Dynamic, 0.3, 0.5);

                body.translate_by(&Vec3::new(x, y, z));

                world.add_object(@mut body::RigidBody(body));
                graphics.add_cube(body, One::one(), &box);
            }
        }
    }

    /*
     * Set up the camera and that is it!
     */
    graphics.look_at(Vec3::new(-30.0, 30.0, -30.0), Vec3::new(0.0, 0.0, 0.0));

    (world, detector, ccd)
}
