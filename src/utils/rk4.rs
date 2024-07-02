use log::debug;

/// Implementation of the fourth order Runge Kutta method.
/// Returns the solution to the problem dx/dt = F(x, u) where u is held constant
/// * `x` - The initial state of the system
/// * `u` - A constant
/// * `t` - The timestep of the system
/// * `f` - The function f(x, u) = dx/dt

pub fn solve_rk4(x: f32, u: f32, t: f32, f: impl Fn(f32, f32) -> f32) -> f32 {
    let init_x = x;
    // The constant u, used to control, is assumed to be constant across the time interval.
    let init_u = u;

    let k1 = f(init_x, init_u);
    let k2 = f(init_x + t * 0.5 * k1, init_u);
    let k3 = f(init_x + t * 0.5 * k2, init_u);
    let k4 = f(init_x + t * k3, init_u);

    debug!(
        "Solving Runge-Kutta, 4th Order, k1: {}, k2: {}, k3: {}, k4: {} solved with timestep {}s, result {}",
        k1,
        k2,
        k3,
        k4,
        t,
        init_x + t * ((k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0)
    );

    init_x + t * ((k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0)
}
