// @generated from registries/qty_aliases.toml, registries/qty_products.toml by xtask::generate::quantity_aliases_and_products (N33c-03); do not edit by hand.

pub const ALIAS_COUNT: usize = 36;

define_qty_aliases! {
    Length = (1, 0, 0, 0, 0, 0), "length", "m";
    Area = (2, 0, 0, 0, 0, 0), "area", "m^2";
    Volume = (3, 0, 0, 0, 0, 0), "volume", "m^3";
    Angle = (0, 0, 0, 0, 0, 0), "angle", "rad";
    Ratio = (0, 0, 0, 0, 0, 0), "ratio", "1";
    Mass = (0, 1, 0, 0, 0, 0), "mass", "kg";
    Density = (-3, 1, 0, 0, 0, 0), "density", "kg·m^-3";
    Time = (0, 0, 1, 0, 0, 0), "time", "s";
    Frequency = (0, 0, -1, 0, 0, 0), "frequency", "Hz";
    AngularVelocity = (0, 0, -1, 0, 0, 0), "angular-velocity", "rad·s^-1";
    AngularAcceleration = (0, 0, -2, 0, 0, 0), "angular-acceleration", "rad·s^-2";
    Velocity = (1, 0, -1, 0, 0, 0), "velocity", "m·s^-1";
    Acceleration = (1, 0, -2, 0, 0, 0), "acceleration", "m·s^-2";
    Force = (1, 1, -2, 0, 0, 0), "force", "N";
    Torque = (2, 1, -2, 0, 0, 0), "torque", "N·m";
    Energy = (2, 1, -2, 0, 0, 0), "energy", "J";
    Power = (2, 1, -3, 0, 0, 0), "power", "W";
    Momentum = (1, 1, -1, 0, 0, 0), "momentum", "kg·m·s^-1";
    Impulse = (1, 1, -1, 0, 0, 0), "impulse", "N·s";
    AngularMomentum = (2, 1, -1, 0, 0, 0), "angular-momentum", "kg·m^2·s^-1";
    MomentOfInertia = (2, 1, 0, 0, 0, 0), "moment-of-inertia", "kg·m^2";
    Stiffness = (0, 1, -2, 0, 0, 0), "stiffness", "N·m^-1";
    AngularStiffness = (2, 1, -2, 0, 0, 0), "angular-stiffness", "N·m·rad^-1";
    Damping = (0, 1, -1, 0, 0, 0), "damping", "N·s·m^-1";
    AngularDamping = (2, 1, -1, 0, 0, 0), "angular-damping", "N·m·s·rad^-1";
    Pressure = (-1, 1, -2, 0, 0, 0), "pressure", "Pa";
    Temperature = (0, 0, 0, 1, 0, 0), "temperature", "K";
    HeatCapacity = (2, 1, -2, -1, 0, 0), "heat-capacity", "J·K^-1";
    ThermalResistance = (-2, -1, 3, 1, 0, 0), "thermal-resistance", "K·W^-1";
    Current = (0, 0, 0, 0, 1, 0), "current", "A";
    Voltage = (2, 1, -3, 0, -1, 0), "voltage", "V";
    Resistance = (2, 1, -3, 0, -2, 0), "resistance", "Ω";
    Inductance = (2, 1, -2, 0, -2, 0), "inductance", "H";
    Charge = (0, 0, 1, 0, 1, 0), "charge", "C";
    TorqueConstant = (2, 1, -2, 0, -1, 0), "torque-constant", "N·m·A^-1";
    FrictionCoefficient = (0, 0, 0, 0, 0, 0), "friction-coefficient", "1";
}

define_qty_ops! {
    Length * Length => Area;
    Area * Length => Volume;
    Length * Area => Volume;
    Density * Volume => Mass;
    Velocity * Time => Length;
    Acceleration * Time => Velocity;
    Mass * Acceleration => Force;
    Mass * Velocity => Momentum;
    Force * Time => Impulse;
    Force * Length => Energy;
    Force * Velocity => Power;
    Torque * AngularVelocity => Power;
    Power * Time => Energy;
    MomentOfInertia * AngularAcceleration => Torque;
    MomentOfInertia * AngularVelocity => AngularMomentum;
    Stiffness * Length => Force;
    Damping * Velocity => Force;
    AngularStiffness * Angle => Torque;
    AngularDamping * AngularVelocity => Torque;
    Pressure * Area => Force;
    Voltage * Current => Power;
    Resistance * Current => Voltage;
    Current * Time => Charge;
    TorqueConstant * Current => Torque;
    ;
    Length / Time => Velocity;
    Velocity / Time => Acceleration;
    Angle / Time => AngularVelocity;
    AngularVelocity / Time => AngularAcceleration;
    Mass / Volume => Density;
    Force / Area => Pressure;
    Energy / Time => Power;
    Voltage / Current => Resistance;
    Power / Current => Voltage;
    Power / Voltage => Current;
    Torque / Current => TorqueConstant;
}
