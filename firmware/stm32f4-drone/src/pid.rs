
#![allow(dead_code)]

use core::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Pid {
    // Gains
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,

    // State
    integral: f32,
    prev_meas: f32,
    has_prev: bool,

    // Limits
    out_min: f32,
    out_max: f32,
    integral_limit: f32,
}

impl Pid {
    pub const fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_meas: 0.0,
            has_prev: false,
            out_min: f32::NEG_INFINITY,
            out_max: f32::INFINITY,
            integral_limit: f32::INFINITY,
        }
    }

    pub const fn with_output_limits(mut self, min: f32, max: f32) -> Self {
        self.out_min = min;
        self.out_max = max;
        self
    }

    pub const fn with_integral_limit(mut self, limit: f32) -> Self {
        self.integral_limit = limit;
        self
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_meas = 0.0;
        self.has_prev = false;
    }

    pub fn update(&mut self, setpoint: f32, measurement: f32, dt_s: f32) -> f32 {
        // Protect against zero/negative dt
        let dt = if dt_s > 0.0 { dt_s } else { return 0.0 };

        // Error
        let error = setpoint - measurement;

        // Proportional
        let p = self.kp * error;

        // Integral with anti-windup (simple clamping)
        self.integral += error * dt * self.ki;
        self.integral = clamp_sym(self.integral, self.integral_limit);
        let i = self.integral;

        // Derivative on measurement to reduce setpoint kick
        let d = if self.has_prev {
            let deriv_meas = (measurement - self.prev_meas) / dt; // d(meas)/dt
            -self.kd * deriv_meas
        } else {
            0.0
        };
        self.prev_meas = measurement;
        self.has_prev = true;

        // Sum and clamp output
        let mut out = p + i + d;
        if out > self.out_max { out = self.out_max; }
        if out < self.out_min { out = self.out_min; }
        out
    }

    pub fn update_with_error(&mut self, error: f32, dt_s: f32) -> f32 {
        let dt = if dt_s > 0.0 { dt_s } else { return 0.0 };
        let p = self.kp * error;
        self.integral += error * dt * self.ki;
        self.integral = clamp_sym(self.integral, self.integral_limit);
        let i = self.integral;
        let mut out = p + i; // no D without measurement
        if out > self.out_max { out = self.out_max; }
        if out < self.out_min { out = self.out_min; }
        out
    }

    pub fn set_gains(&mut self, kp: f32, ki: f32, kd: f32) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }
}

#[inline]
fn clamp_sym(x: f32, lim: f32) -> f32 {
    if lim.is_finite() {
        if x > lim { return lim; }
        if x < -lim { return -lim; }
    }
    x
}
