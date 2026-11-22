/**
 * The spring-mass runner: a point mass on a massless springy leg that is set to the same angle before every
 * landing (Seyfarth, Geyer, Günther and Blickhan, "A movement criterion for running", J. Biomechanics 35, 2002).
 * One runner per cell of a grid of landing angle by leg stiffness, all released from the same apex.
 */
export const MASS = 80;
export const REST = 1;
export const G = 9.81;
export const SPEED = 5;
export const APEX = 1;

/** Grid axes: landing angle from the ground in degrees, leg stiffness in kN/m. */
export const ANGLE = [56, 80] as const;
export const STIFF = [5, 55] as const;

export const FLIGHT = 0;
export const STANCE = 1;
export const FELL = 2;

export class Field {
  readonly count: number;
  readonly x: Float32Array;
  readonly y: Float32Array;
  readonly vx: Float32Array;
  readonly vy: Float32Array;
  /** The foothold of the current or latest stance, in the same frame as x. */
  readonly foot: Float32Array;
  readonly state: Uint8Array;
  readonly steps: Uint16Array;
  readonly fellAt: Float32Array;
  readonly k: Float32Array;
  readonly cos: Float32Array;
  readonly sin: Float32Array;
  time = 0;
  standing: number;

  constructor(
    readonly cols: number,
    readonly rows: number,
  ) {
    const n = (this.count = cols * rows);
    this.x = new Float32Array(n);
    this.y = new Float32Array(n);
    this.vx = new Float32Array(n);
    this.vy = new Float32Array(n);
    this.foot = new Float32Array(n);
    this.state = new Uint8Array(n);
    this.steps = new Uint16Array(n);
    this.fellAt = new Float32Array(n);
    this.k = new Float32Array(n);
    this.cos = new Float32Array(n);
    this.sin = new Float32Array(n);
    for (let row = 0; row < rows; row++) {
      for (let col = 0; col < cols; col++) {
        const i = row * cols + col;
        const a = ((ANGLE[0] + ((ANGLE[1] - ANGLE[0]) * (col + 0.5)) / cols) * Math.PI) / 180;
        // Row 0 is the top of the map, where the legs are stiffest.
        this.k[i] = 1000 * (STIFF[1] - ((STIFF[1] - STIFF[0]) * (row + 0.5)) / rows);
        this.cos[i] = Math.cos(a);
        this.sin[i] = Math.sin(a);
      }
    }
    this.standing = n;
    this.release();
  }

  release() {
    this.x.fill(0);
    this.y.fill(APEX);
    this.vx.fill(SPEED);
    this.vy.fill(0);
    this.foot.fill(0);
    this.state.fill(FLIGHT);
    this.steps.fill(0);
    this.fellAt.fill(0);
    this.time = 0;
    this.standing = this.count;
  }

  /** Advances every runner by dt seconds, split into `sub` velocity-Verlet steps. */
  advance(dt: number, sub: number) {
    const h = dt / sub;
    const { x, y, vx, vy, foot, state, steps, fellAt, k, cos, sin } = this;
    for (let i = 0; i < this.count; i++) {
      let s = state[i]!;
      if (s === FELL) continue;
      let px = x[i]!;
      let py = y[i]!;
      let ux = vx[i]!;
      let uy = vy[i]!;
      let fx = foot[i]!;
      const kk = k[i]!;
      const td = REST * sin[i]!;
      for (let j = 0; j < sub; j++) {
        if (s === FLIGHT) {
          // The flight is a parabola, so its step is exact.
          px += ux * h;
          py += uy * h - 0.5 * G * h * h;
          uy -= G * h;
          if (uy < 0 && py <= td) {
            fx = px + REST * cos[i]!;
            s = STANCE;
          }
        } else {
          let dx = px - fx;
          let len = Math.hypot(dx, py);
          let f = ((kk / MASS) * (REST - len)) / len;
          ux += 0.5 * h * f * dx;
          uy += 0.5 * h * (f * py - G);
          px += h * ux;
          py += h * uy;
          dx = px - fx;
          len = Math.hypot(dx, py);
          f = ((kk / MASS) * (REST - len)) / len;
          ux += 0.5 * h * f * dx;
          uy += 0.5 * h * (f * py - G);
          if (py <= 0.05 || ux <= 0) {
            s = FELL;
            break;
          }
          if (len >= REST && dx * ux + py * uy > 0) {
            s = FLIGHT;
            steps[i]!++;
            // A hop too low to set the leg at its landing angle ends the run.
            const top = py + (uy > 0 ? (uy * uy) / (2 * G) : 0);
            if (top <= td) {
              s = FELL;
              break;
            }
          }
        }
      }
      x[i] = px;
      y[i] = py;
      vx[i] = ux;
      vy[i] = uy;
      foot[i] = fx;
      if (s === FELL) {
        fellAt[i] = this.time;
        this.standing--;
      }
      state[i] = s;
    }
    this.time += dt;
  }
}
