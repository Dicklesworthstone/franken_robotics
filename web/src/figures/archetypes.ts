/** The eighteen archetypes of §14.10: layout, actuated coordinates and symmetry as the plan's table gives them. */
export interface Archetype {
  id: string;
  dof: number;
  symmetry: string;
  layout: string;
  tests: string;
}

export const archetypes: Archetype[] = [
  {
    id: "quad-mammal",
    dof: 12,
    symmetry: "C₂ × C₂",
    layout: "Dog-like quadruped, three joints per leg.",
    tests:
      "Floating-base dynamics, the gait atlas with trot, pace and bound, and knee heat on a stair task.",
  },
  {
    id: "quad-sprawl",
    dof: 12,
    symmetry: "C₂ × C₂",
    layout: "Sprawled quadruped with a wide stance.",
    tests:
      "Large static margins against high holding torque. The design is stable and still fails on heat.",
  },
  {
    id: "hexapod-radial",
    dof: 18,
    symmetry: "D₆",
    layout: "Radial hexapod, three joints per leg.",
    tests:
      "Character tables and orbit counts for D₆, the tripod gait, and static walking with three feet down.",
  },
  {
    id: "octopod",
    dof: 24,
    symmetry: "D₈",
    layout: "Eight-legged, three joints per leg. A spider layout keeps only C₂.",
    tests: "Contact capacity with eight feet, and what is left when a leg fails.",
  },
  {
    id: "biped-point",
    dof: 6,
    symmetry: "C₂",
    layout: "Biped on point feet.",
    tests:
      "Underactuation and impact, with no static stability at all. The hardest case for the baseline controller.",
  },
  {
    id: "humanoid",
    dof: 29,
    symmetry: "C₂",
    layout: "Humanoid with arms and a waist.",
    tests:
      "The upper end of the dense mass matrix, coupled ankle and waist transmissions, and self-collision over large ranges.",
  },
  {
    id: "diff-drive",
    dof: 2,
    symmetry: "C₂",
    layout: "Two driven wheels and a caster.",
    tests:
      "Growth vector (2, 3), the sideways cost of a base that cannot slide, and the lever arm of a wheel at a raised threshold.",
  },
  {
    id: "ackermann",
    dof: 3,
    symmetry: "C₂",
    layout: "Car-like, with the front axle steered through a linkage.",
    tests: "Closure and mobility of the steering linkage, and a turning radius of L / tan δ.",
  },
  {
    id: "mecanum",
    dof: 4,
    symmetry: "C₂ × C₂",
    layout: "Four mecanum wheels.",
    tests:
      "The roller contact law, with a negative control: all rollers parallel, which the preflight must refuse.",
  },
  {
    id: "omni-3",
    dof: 3,
    symmetry: "D₃",
    layout: "Three omni wheels at 120°.",
    tests: "A holonomic base with an odd rotation order, and the inverse of its wheel map.",
  },
  {
    id: "swerve-4",
    dof: 8,
    symmetry: "D₄",
    layout: "Four modules that steer and drive.",
    tests:
      "Nested symmetry of module and hub, scrub torque, and the singular pose with all wheel axes through one point.",
  },
  {
    id: "tracked-flipper",
    dof: 6,
    symmetry: "C₂ × C₂",
    layout: "Two tracks with front and rear flippers.",
    tests:
      "The belt law and skid steering, where turning in place must cost scrub work, and stair nosing.",
  },
  {
    id: "wheel-leg",
    dof: 16,
    symmetry: "C₂ × C₂",
    layout: "Quadruped with driven wheels for feet.",
    tests: "Two contact laws on one limb, and switching between rolling and stepping.",
  },
  {
    id: "tri-wheel-stair",
    dof: 4,
    symmetry: "C₂",
    layout: "Two clusters of three wheels on a chassis.",
    tests: "Cluster geometry against riser and tread: a three-fold group inside a mirror group.",
  },
  {
    id: "rocker-bogie",
    dof: 6,
    symmetry: "C₂",
    layout: "Six wheels on a passive rocker-bogie suspension.",
    tests: "Passive joints, a differential closure, and climbing without actuated suspension.",
  },
  {
    id: "mobile-manipulator",
    dof: 9,
    symmetry: "Trivial",
    layout: "Seven-joint arm, off-centre, on a wheeled base.",
    tests:
      "Workspace and singular poses at tray and shelf heights, and a payload that breaks the symmetry.",
  },
  {
    id: "snake-crawler",
    dof: 8,
    symmetry: "C₂",
    layout: "Planar serial chain on passive wheels.",
    tests: "Locomotion as holonomy: net motion from a closed loop of shapes.",
  },
  {
    id: "balancer-2w",
    dof: 2,
    symmetry: "C₂",
    layout: "Two-wheeled inverted pendulum.",
    tests:
      "An unstable equilibrium, so the estimator and the baseline controller must work on the first step.",
  },
];
