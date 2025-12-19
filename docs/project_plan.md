# Project Plan: Mini Drone MVP

This document outlines the project plan, including the timeline, key milestones, and risk assessment for the development of the mini drone MVP.

---

## 1. Project Timeline & Milestones

The project was executed over a 14-week period, broken down into distinct phases from research to final presentation.

| Weeks   | Phase                            | Key Milestones & Goals                   | Status |
| :------ | :------------------------------- | :--------------------------------------- | :----- |
| **1-2** | **Phase 1: Research & Planning** | - Define project scope and MVP features. |

- Research flight controllers and components.
- Finalize team roles. | ✅ Complete |
  | **3-4** | **Phase 2: Hardware Acquisition & Assembly** | - Source and purchase all components.
- Assemble the drone frame.
- Solder motors, ESCs, and PDB. | ✅ Complete |
  | **5-7** | **Phase 3: Firmware & Initial Setup** | - Flash flight controller with Betaflight firmware.
- Bind the receiver to the transmitter.
- Calibrate ESCs and test motor direction. | ✅ Complete |
  | **8-10** | **Phase 4: Programming & Tuning** | - Configure flight modes (Angle, Horizon).
- Perform initial PID tuning for stability.
- Implement safety features (e.g., arming sequence). | ✅ Complete |
  | **11-12** | **Phase 5: Flight Testing & Iteration** | - Conduct first hover tests.
- Gather feedback on flight characteristics.
- Refine PID settings based on test flight data. | ✅ Complete |
  | **13-14** | **Phase 6: Finalization** | - Record backup demo video.
- Finalize all documentation (README, etc.).
- Prepare and rehearse the final presentation. | ✅ Complete |

---

## 2. Risk Management

We identified potential risks early in the project and planned mitigation strategies to minimize their impact.

| Risk ID                                                            | Risk Description                                                        | Probability | Impact | Mitigation Strategy                                                                                               |
| :----------------------------------------------------------------- | :---------------------------------------------------------------------- | :---------- | :----- | :---------------------------------------------------------------------------------------------------------------- |
| **R-01**                                                           | **Component Failure:** Motor, ESC, or FC burns out during testing.      | Medium      | High   | - Ordered spare motors and ESCs.                                                                                  |
| - Used a current-limiting "smoke stopper" during initial power-up. |
| **R-02**                                                           | **Unstable Flight:** Drone is unflyable due to oscillations or drift.   | High        | High   | - Allocated significant time (3 weeks) for PID tuning and sensor calibration.                                     |
| - Watched tutorials on tuning theory.                              |
| **R-03**                                                           | **Demo Failure:** Technical issues prevent a live demo during the exam. | Low         | High   | - Recorded a high-quality backup video of a successful flight demonstration.                                      |
| - Charged all batteries fully before the exam.                     |
| **R-04**                                                           | **Supply Chain Delays:** Components do not arrive on time.              | Medium      | Medium | - Ordered all components from reputable suppliers with clear shipping estimates well ahead of the assembly phase. |
| **R-05**                                                           | **Software Bugs:** Firmware issues or configuration conflicts.          | High        | Medium | - Used a stable, widely-supported version of the firmware.                                                        |
| - Saved configuration backups at each major step.                  |
