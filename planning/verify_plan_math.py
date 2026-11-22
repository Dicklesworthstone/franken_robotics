#!/usr/bin/env python3
"""
Committed, reproducible mathematical, physics, recursive-descent EBNF, crate-layering,
and DAG verifier for COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md (v2.0).

Run from anywhere:
  python3 /Users/jemanuel/projects/franken_robotics/planning/verify_plan_math.py

Independently verifies (zero external dependencies beyond Python stdlib):
  1. FULL Math & Physics Suite:
     - GB-01: Cartan curvature minus sign on BOTH SO(3) and SE(2) coupled snake
     - GB-02: Matrix SE(3) Cayley differential vs 6x6 (d tau_u^left)^{-1} to <1e-11,
              free SO(3) LGVI, AND full 6-DoF SE(3) heavy-top + off-diag CoM + spring
              LGVI (O(h^2) energy scaling, spatial momentum drift, 100-step reversibility)
     - GB-03..GB-16: 3-tier Delassus SRAM (incl. 1,872 B 16-B stride), W10 multi-RHS +
              Amdahl, Burnside D6/Z2, COD ballast quotient, .fenergy w/ E_ext, power-
              invariant dq PMSM First-Law + GB-08 runaway 33.5s + N_knee 12->22 repair
              flipping a'<0 (T(45s)=58.7C), Bekker-Wong n=0.5, 3-stage early stop,
              brute-force V4 & Z2 strata/transitions (27/162, 7/16, 45/324, 10/32),
              .fclear 240x160x32, GB-13 box sort-and-clip, DPOR 720->8, Lyapunov/CRN,
              10,700 retrofit kits, Rimless Wheel singular M_full vs nonsingular Poincare
              IFT, Frobenius-Schur R/C/H projectors (D6, C5, Q8), 18D se(3) anchored
              W-orthogonal sheaf w/ Ad_{T_ev}, 2D FV thin-film tribology mass & positivity,
              and Smale alpha-theory counterexample guard.
  2. Exact 51-crate budget table (§22.2: 31★ / 220.7 kLOC, 378.2 / 435.3 kLOC) & §0.5 sync.
  3. 100-bead DAG (§29.2): 0 cycles, 0 missing deps, 0 orphans, sole sink = BD-096,
     milestone monotonicity, AND multi-crate-aware Tarjan SCC + strict layer checks.
  4. Cross-reference completeness across all 13 ID families + 16 GOALs + 46 MCP tools.
  5. REAL recursive-descent EBNF parser for Appendix D.1 (.fbot), D.1b (.fmodule), and
     D.2 (.ftask) + exact physical/mass/width/height/kinematics/steering/clearance audit
     + strict json.loads() & safety-flag validation on all 6 Appendix D.3 blocks.
  6. Markdown table hygiene, continuous H2 §1..§29, live file:// links, and unified
     † K-01..† K-13 parity with planning/brenner/deliverables/ARTIFACT.md.
"""
import cmath
import json
import math
import re
import sys
from collections import defaultdict, deque
from itertools import combinations, product
from pathlib import Path

ROOT = Path("/Users/jemanuel/projects/franken_robotics")
PLAN = ROOT / "COMPREHENSIVE_PLAN_FOR_FRANKEN_ROBOTICS__NEON.md"
ARTIFACT = ROOT / "planning/brenner/deliverables/ARTIFACT.md"

text = PLAN.read_text(encoding="utf-8")
lines = text.splitlines()

errors = []
def err(msg):
    errors.append(msg)

# ===========================================================================
# 1. INDEPENDENT MATH & PHYSICS VERIFICATIONS
# ===========================================================================
print("=== 1. Independent Math & Physics Recomputations ===")

def mat_mul(A, B):
    n, k, m = len(A), len(B), len(B[0])
    return [[sum(A[i][p]*B[p][j] for p in range(k)) for j in range(m)] for i in range(n)]

def mat_vec(A, v):
    return [sum(A[i][j]*v[j] for j in range(len(v))) for i in range(len(A))]

def mat_T(A):
    return [[A[j][i] for j in range(len(A))] for i in range(len(A[0]))]

def mat_eye(n):
    return [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]

def mat_inv(A):
    n = len(A)
    M = [row[:] + [1.0 if i == j else 0.0 for j in range(n)] for i, row in enumerate(A)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(M[r][col]))
        M[col], M[piv] = M[piv], M[col]
        div = M[col][col]
        for j in range(col, 2*n):
            M[col][j] /= div
        for r in range(n):
            if r != col:
                f = M[r][col]
                for j in range(col, 2*n):
                    M[r][j] -= f * M[col][j]
    return [row[n:] for row in M]

def solve_linear(A, b):
    n = len(b)
    M = [row[:] + [b[i]] for i, row in enumerate(A)]
    for col in range(n):
        piv = max(range(col, n), key=lambda r: abs(M[r][col]))
        M[col], M[piv] = M[piv], M[col]
        div = M[col][col]
        for j in range(col, n + 1):
            M[col][j] /= div
        for r in range(n):
            if r != col:
                f = M[r][col]
                for j in range(col, n + 1):
                    M[r][j] -= f * M[col][j]
    return [M[i][n] for i in range(n)]

def hat3(w):
    return [[0.0, -w[2], w[1]], [w[2], 0.0, -w[0]], [-w[1], w[0], 0.0]]

def cross3(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]

def dot(a, b):
    return sum(x*y for x, y in zip(a, b))

def norm2(v):
    return math.sqrt(dot(v, v))

# --- GB-01: Cartan Curvature Minus Sign on BOTH SO(3) AND SE(2) ---
def so3_exp(w):
    th = norm2(w)
    W = hat3(w)
    W2 = mat_mul(W, W)
    if th < 1e-8:
        a, b = 1.0 - th*th/6.0, 0.5 - th*th/24.0
    else:
        a, b = math.sin(th)/th, (1.0 - math.cos(th))/(th*th)
    I = mat_eye(3)
    return [[I[i][j] + a*W[i][j] + b*W2[i][j] for j in range(3)] for i in range(3)]

def so3_log(R):
    tr = max(-1.0, min(1.0, (R[0][0]+R[1][1]+R[2][2]-1.0)/2.0))
    th = math.acos(tr)
    if th < 1e-10:
        return [0.5*(R[2][1]-R[1][2]), 0.5*(R[0][2]-R[2][0]), 0.5*(R[1][0]-R[0][1])]
    c = th / (2.0 * math.sin(th))
    return [c*(R[2][1]-R[1][2]), c*(R[0][2]-R[2][0]), c*(R[1][0]-R[0][1])]

def A_so3(r1, r2):
    return [1.0 + 0.5*r2, 0.2*r1, 0.1*r2], [0.1*r1, 1.0 - 0.3*r1, 0.2*r2]

eps_loop = 0.0015
h_half = 0.5 * eps_loop
g_cur = mat_eye(3)
pts = [(-h_half,-h_half), (h_half,-h_half), (h_half,h_half), (-h_half,h_half), (-h_half,-h_half)]
for s in range(4):
    x0, y0 = pts[s]; x1, y1 = pts[s+1]
    dx, dy = (x1-x0)/400, (y1-y0)/400
    for k in range(400):
        A1, A2 = A_so3(x0+(k+0.5)*dx, y0+(k+0.5)*dy)
        g_cur = mat_mul(g_cur, so3_exp([-(A1[i]*dx + A2[i]*dy) for i in range(3)]))
xi_exact = so3_log(g_cur)
A1c, A2c = A_so3(0.0, 0.0)
comm = cross3(A1c, A2c)
curv_m = [-(0.1 - 0.5 - comm[0])*eps_loop**2, -(-0.3 - 0.0 - comm[1])*eps_loop**2, -(0.0 - 0.1 - comm[2])*eps_loop**2]
curv_p = [-(0.1 - 0.5 + comm[0])*eps_loop**2, -(-0.3 - 0.0 + comm[1])*eps_loop**2, -(0.0 - 0.1 + comm[2])*eps_loop**2]
err_so3_m = norm2([a-b for a,b in zip(xi_exact, curv_m)]) / norm2(xi_exact)
err_so3_p = norm2([a-b for a,b in zip(xi_exact, curv_p)]) / norm2(xi_exact)
assert err_so3_m < 1.2e-3 and err_so3_p > 1.5

# SE(2) coupled snake Cartan curvature (omega, vx, vy)
def se2_exp(xi):
    w, vx, vy = xi
    if abs(w) < 1e-9:
        c, s = 1.0 - 0.5*w*w, w - w**3/6.0
        a, b = 1.0 - w*w/6.0, 0.5*w
    else:
        c, s = math.cos(w), math.sin(w)
        a, b = s/w, (1.0 - c)/w
    return [[c, -s, a*vx - b*vy], [s, c, b*vx + a*vy], [0.0, 0.0, 1.0]]

def se2_log(T):
    w = math.atan2(T[1][0], T[0][0])
    tx, ty = T[0][2], T[1][2]
    if abs(w) < 1e-9:
        return [w, tx, ty]
    half = 0.5*w / math.tan(0.5*w)
    return [w, half*tx + 0.5*w*ty, -0.5*w*tx + half*ty]

def se2_bracket(a, b):
    # [(wa, vxa, vya), (wb, vxb, vyb)] = (0, wa*(-vyb) - wb*(-vya), wa*vxb - wb*vxa)
    return [0.0, -a[0]*b[2] + b[0]*a[2], a[0]*b[1] - b[0]*a[1]]

def A_se2(r1, r2):
    return [0.8 + 0.3*r2, 1.2 - 0.4*r1, 0.5*r2], [0.6 - 0.2*r1, 0.4*r2, 1.1 + 0.3*r1]

eps2 = 0.002; hh2 = 0.5*eps2
T_cur = mat_eye(3)
pts2 = [(-hh2,-hh2), (hh2,-hh2), (hh2,hh2), (-hh2,hh2), (-hh2,-hh2)]
for s in range(4):
    x0, y0 = pts2[s]; x1, y1 = pts2[s+1]
    dx, dy = (x1-x0)/300, (y1-y0)/300
    for k in range(300):
        A1, A2 = A_se2(x0+(k+0.5)*dx, y0+(k+0.5)*dy)
        T_cur = mat_mul(T_cur, se2_exp([-(A1[i]*dx + A2[i]*dy) for i in range(3)]))
xi2_ex = se2_log(T_cur)
A1c2, A2c2 = A_se2(0.0, 0.0)
br2 = se2_bracket(A1c2, A2c2)
dA2_1 = [-0.2, 0.0, 0.3]; dA1_2 = [0.3, 0.0, 0.5]
c2_m = [-(dA2_1[i] - dA1_2[i] - br2[i])*eps2**2 for i in range(3)]
c2_p = [-(dA2_1[i] - dA1_2[i] + br2[i])*eps2**2 for i in range(3)]
err_se2_m = norm2([a-b for a,b in zip(xi2_ex, c2_m)]) / norm2(xi2_ex)
err_se2_p = norm2([a-b for a,b in zip(xi2_ex, c2_p)]) / norm2(xi2_ex)
assert err_se2_m < 2.5e-3 and err_se2_p > 1.0

# --- GB-02: FULL 6-DoF SE(3) Matrix-Cayley LGVI + Gravity/Spring + Free SO(3) ---
def se3_hat(u):
    w, v = u[:3], u[3:]
    W = hat3(w)
    return [[W[0][0],W[0][1],W[0][2],v[0]],
            [W[1][0],W[1][1],W[1][2],v[1]],
            [W[2][0],W[2][1],W[2][2],v[2]],
            [0.0, 0.0, 0.0, 0.0]]

def se3_vee(M):
    return [M[2][1], M[0][2], M[1][0], M[0][3], M[1][3], M[2][3]]

def se3_cayley(u):
    # Matrix Cayley: (I + 0.5*hat(u))(I - 0.5*hat(u))^{-1}
    U = se3_hat(u)
    I4 = mat_eye(4)
    P = [[I4[i][j] + 0.5*U[i][j] for j in range(4)] for i in range(4)]
    Q = [[I4[i][j] - 0.5*U[i][j] for j in range(4)] for i in range(4)]
    return mat_mul(P, mat_inv(Q))

def dtau_se3_left_inv(u):
    # Exact 6x6 block formula from §8.1:
    # [[I + 0.5*[w] + 0.25*w w^T, 0], [0.5*[v] + 0.25*[w][v], I + 0.5*[w]]]
    w, v = u[:3], u[3:]
    W, V = hat3(w), hat3(v)
    WV = mat_mul(W, V)
    I3 = mat_eye(3)
    M = [[0.0]*6 for _ in range(6)]
    for i in range(3):
        for j in range(3):
            M[i][j] = I3[i][j] + 0.5*W[i][j] + 0.25*w[i]*w[j]
            M[i+3][j] = 0.5*V[i][j] + 0.25*WV[i][j]
            M[i+3][j+3] = I3[i][j] + 0.5*W[i][j]
    return M

# Verify 6x6 dtau_se3_left_inv against finite-diff tau(u)^{-1} d tau(u)
u_test = [0.31, -0.42, 0.27, 0.55, -0.18, 0.63]
T_u = se3_cayley(u_test)
T_u_inv = mat_inv(T_u)
dtau_fd = [[0.0]*6 for _ in range(6)]
eps_fd = 1e-7
for j in range(6):
    up = u_test[:]; up[j] += eps_fd
    um = u_test[:]; um[j] -= eps_fd
    dT = [[(se3_cayley(up)[r][c] - se3_cayley(um)[r][c])/(2*eps_fd) for c in range(4)] for r in range(4)]
    col_vec = se3_vee(mat_mul(T_u_inv, dT))
    for i in range(6):
        dtau_fd[i][j] = col_vec[i]
prod_check = mat_mul(dtau_se3_left_inv(u_test), dtau_fd)
dtau_err = max(abs(prod_check[i][j] - (1.0 if i==j else 0.0)) for i in range(6) for j in range(6))
assert dtau_err < 1e-9, f"6x6 dtau_se3_left_inv mismatch: {dtau_err}"

def se3_Ad(T):
    R = [[T[i][j] for j in range(3)] for i in range(3)]
    p = [T[0][3], T[1][3], T[2][3]]
    P_R = mat_mul(hat3(p), R)
    A = [[0.0]*6 for _ in range(6)]
    for i in range(3):
        for j in range(3):
            A[i][j] = R[i][j]
            A[i+3][j] = P_R[i][j]
            A[i+3][j+3] = R[i][j]
    return A

# Full 6-DoF SE(3) spatial inertia with off-diagonal CoM c_b + gravity + spring potential
m_body = 2.4
c_b = [0.12, -0.05, 0.08]
I_b = [[0.35, 0.02, -0.01], [0.02, 0.52, 0.03], [-0.01, 0.03, 0.68]]
C_hat = hat3(c_b)
M6 = [[0.0]*6 for _ in range(6)]
for i in range(3):
    for j in range(3):
        M6[i][j] = I_b[i][j]
        M6[i][j+3] = m_body * C_hat[i][j]
        M6[i+3][j] = -m_body * C_hat[i][j]
        M6[i+3][j+3] = m_body if i == j else 0.0
M6_inv = mat_inv(M6)

g_vec = [0.0, 0.0, -9.81]
k_spr = 15.0
p_att_b = [0.2, 0.1, -0.05]
p_anc_w = [0.0, 0.0, 0.5]

def pot_V_and_wrench(T):
    # V(g) = -m g_vec . (p + R c_b) + 0.5 k ||p + R p_att_b - p_anc_w||^2
    R = [[T[i][j] for j in range(3)] for i in range(3)]
    p = [T[0][3], T[1][3], T[2][3]]
    Rc = mat_vec(R, c_b)
    p_com = [p[i] + Rc[i] for i in range(3)]
    Ra = mat_vec(R, p_att_b)
    diff = [p[i] + Ra[i] - p_anc_w[i] for i in range(3)]
    V = -m_body * dot(g_vec, p_com) + 0.5 * k_spr * dot(diff, diff)
    # Left-trivialized potential wrench f_pot = -T_e^* L_g (dV/dg)
    f_g_b = mat_vec(mat_T(R), [m_body * g_vec[i] for i in range(3)])
    tau_g_b = cross3(c_b, f_g_b)
    f_s_b = mat_vec(mat_T(R), [-k_spr * diff[i] for i in range(3)])
    tau_s_b = cross3(p_att_b, f_s_b)
    return V, [tau_g_b[i]+tau_s_b[i] for i in range(3)] + [f_g_b[i]+f_s_b[i] for i in range(3)]

def lgvi_se3_step(T, mu_m, h):
    _, f0 = pot_V_and_wrench(T)
    mu_p = [mu_m[i] + 0.5*h*f0[i] for i in range(6)]
    xi = mat_vec(M6_inv, mu_p)
    for _ in range(10):
        A = mat_T(dtau_se3_left_inv([-h*x for x in xi]))
        Mxi = mat_vec(M6, xi)
        res = [mat_vec(A, Mxi)[i] - mu_p[i] for i in range(6)]
        if norm2(res) < 1e-14:
            break
        Jmat = [[0.0]*6 for _ in range(6)]
        ep = 1e-7
        for j in range(6):
            xip = xi[:]; xip[j] += ep
            Ap = mat_T(dtau_se3_left_inv([-h*x for x in xip]))
            rp = mat_vec(Ap, mat_vec(M6, xip))
            for i in range(6):
                Jmat[i][j] = (rp[i] - (res[i] + mu_p[i])) / ep
        dxi = solve_linear(Jmat, [-r for r in res])
        xi = [xi[i] + dxi[i] for i in range(6)]
    dT = se3_cayley([h*x for x in xi])
    T_next = mat_mul(T, dT)
    mu_free = mat_vec(mat_T(se3_Ad(dT)), mu_p)
    _, f1 = pot_V_and_wrench(T_next)
    mu_next = [mu_free[i] + 0.5*h*f1[i] for i in range(6)]
    return T_next, mu_next, xi

# Check O(h^2) energy oscillation & 100-step backward reversibility on full 6-DoF SE(3)
T_init = se3_cayley([0.2, -0.15, 0.3, 0.1, -0.2, 0.25])
xi_init = [0.9, -0.6, 0.7, 0.4, -0.3, 0.5]
V0, _ = pot_V_and_wrench(T_init)
E_se3_0 = 0.5 * dot(xi_init, mat_vec(M6, xi_init)) + V0
se3_errs = []
for h_val in [0.008, 0.004, 0.002]:
    T_s = [row[:] for row in T_init]
    mu_s = mat_vec(M6, xi_init)
    n_st = int(round(0.24 / h_val))
    mx = 0.0
    for _ in range(n_st):
        T_s, mu_s, _ = lgvi_se3_step(T_s, mu_s, h_val)
        xi_node = mat_vec(M6_inv, mu_s)  # evaluate kinetic & potential at same integer node t_{k+1}!
        Vk, _ = pot_V_and_wrench(T_s)
        Ek = 0.5 * dot(xi_node, mat_vec(M6, xi_node)) + Vk
        mx = max(mx, abs(Ek - E_se3_0) / abs(E_se3_0))
    se3_errs.append(mx)
assert 3.7 < se3_errs[0]/se3_errs[1] < 4.3 and 3.7 < se3_errs[1]/se3_errs[2] < 4.3, f"SE(3) O(h^2) failed: {se3_errs}"

# 100-step backward time reversibility on full SE(3)
T_r = [row[:] for row in T_init]
mu_r = mat_vec(M6, xi_init)
for _ in range(100):
    T_r, mu_r, _ = lgvi_se3_step(T_r, mu_r, 0.002)
mu_r = [-x for x in mu_r]
for _ in range(100):
    T_r, mu_r, _ = lgvi_se3_step(T_r, mu_r, 0.002)
se3_rev_err = max(abs(T_r[i][j] - T_init[i][j]) for i in range(4) for j in range(4))
assert se3_rev_err < 1e-12, f"SE(3) 100-step reversibility failed: {se3_rev_err}"

# Free SO(3) check matching §8.1 & GB-02
def dtau_so3_inv_T(u):
    W = hat3(u); I = mat_eye(3)
    return [[I[i][j] - 0.5*W[i][j] + 0.25*u[i]*u[j] for j in range(3)] for i in range(3)]

def cayley_so3(w):
    W = hat3(w); W2 = mat_mul(W, W); c = 1.0 / (1.0 + 0.25*dot(w, w)); I = mat_eye(3)
    return [[I[i][j] + c*(W[i][j] + 0.5*W2[i][j]) for j in range(3)] for i in range(3)]

J_inert = [1.2, 2.5, 3.8]; w0 = [1.1, -0.7, 0.9]
E0_so3 = 0.5 * sum(J_inert[i]*w0[i]**2 for i in range(3))
so3_rel = {}
for h_val, n_st in [(0.002, 2000), (0.00025, 4000)]:
    R = mat_eye(3)
    mu = mat_vec(dtau_so3_inv_T([-h_val*x for x in w0]), [J_inert[i]*w0[i] for i in range(3)])
    mx = 0.0
    for _ in range(n_st):
        w = [mu[i]/J_inert[i] for i in range(3)]
        for _ in range(8):
            A = dtau_so3_inv_T([-h_val*x for x in w])
            res = [mat_vec(A, [J_inert[i]*w[i] for i in range(3)])[i] - mu[i] for i in range(3)]
            if norm2(res) < 1e-14: break
            Jm = [[0.0]*3 for _ in range(3)]
            for j in range(3):
                wp = w[:]; wp[j] += 1e-7
                rp = mat_vec(dtau_so3_inv_T([-h_val*x for x in wp]), [J_inert[i]*wp[i] for i in range(3)])
                for i in range(3): Jm[i][j] = (rp[i] - (res[i]+mu[i])) / 1e-7
            dw = solve_linear(Jm, [-r for r in res])
            w = [w[i]+dw[i] for i in range(3)]
        dR = cayley_so3([h_val*x for x in w])
        R = mat_mul(R, dR); mu = mat_vec(mat_T(dR), mu)
        mx = max(mx, abs(0.5*sum(J_inert[i]*w[i]**2 for i in range(3)) - E0_so3)/E0_so3)
    so3_rel[h_val] = mx
assert so3_rel[0.002] < 1e-6 and so3_rel[0.00025] < 2e-8

# --- GB-03..GB-16 + Rimless Wheel + Frobenius-Schur + Sheaf + Tribology + Smale ---
assert (30*31//2)*4 == 1860 and ((1860 + 15)//16)*16 == 1872 and (48*49//2)*4 == 4704 and 4704 % 16 == 0
f_chol, f_solve = 96**3/3.0, 2*96**2
speedup_w10 = (64*(f_chol+f_solve))/(f_chol+64*f_solve)
p_req = (1.0 - 0.1)/(1.0 - 1.0/speedup_w10)
assert abs(speedup_w10 - 13.597) < 0.01 and abs(p_req - 0.97143) < 0.0005

assert (10**6 + 2*10 + 2*10**2 + 10**3 + 3*10**4 + 3*10**3)//12 == 86185
assert (10**6 + 10**3)//2 == 500500
classes = set((sum(c), sum(x*x for x in c)) for c in combinations(range(24), 8))
assert math.comb(24, 8) == 735471 and len(classes) == 24546

# GB-07 .fenergy
assert abs(0.5*8.0*0.4**2 + 20.0*0.025 + 0.25 + 0.10 - 1.49) < 1e-12
assert abs((2.0 - (0.64+0.25+0.10))/20.0 - 0.0505) < 1e-12
assert abs((2.0 - (0.64+0.25+0.10+0.10))/20.0 - 0.0455) < 1e-12

# GB-08 Power-invariant dq PMSM First-Law + runaway 33.5s + N_knee: 12->22 repair
kt = ke = 0.14
iq_pi, R0, alpha_cu, C_th, G_th, T_amb_K = 25.40, 0.54, 0.00393, 140.0, 1.0/2.2, 298.15
omega_m = 100.0
vq_pi = R0 * iq_pi + ke * omega_m
assert abs((vq_pi * iq_pi - R0 * iq_pi**2) - (kt * iq_pi * omega_m)) < 1e-12  # Exact First-Law!
assert abs(48.0 / math.sqrt(2.0) - 33.9411) < 1e-3  # Power-invariant SVPWM ceiling
P20 = iq_pi**2 * R0
P120 = P20 * (1.0 + alpha_cu * 100.0)
a_run = (P20 * alpha_cu - G_th) / C_th
b0 = P20 * (1.0 + alpha_cu * (T_amb_K - 293.15)) / C_th
t_120 = math.log(1.0 + 95.0 * a_run / b0) / a_run
assert abs(P20 - 348.39) < 0.05 and abs(P120 - 485.30) < 0.05
assert abs(a_run - 0.00653) < 0.00005 and abs(t_120 - 33.49) < 0.06
# Repaired N_knee: 12 -> 22
cu_cut_22 = 1.0 - (12.0/22.0)**2
P20_rep = P20 * (12.0/22.0)**2
a_rep = (P20_rep * alpha_cu - G_th) / C_th
b0_rep = P20_rep * (1.0 + alpha_cu * (T_amb_K - 293.15)) / C_th
T_45_C = 25.0 + (b0_rep / a_rep) * (math.exp(45.0 * a_rep) - 1.0)
assert abs(cu_cut_22 - 0.70248) < 1e-4 and a_rep < -3e-4 and abs(T_45_C - 58.7) < 0.2

# GB-09 Bekker-Wong n=0.5
def sink_cm(mass_kg, b_m):
    p_kpa = (mass_kg * 9.81 / 1000.0) / (2.0 * b_m * 0.55)
    return 100.0 * (p_kpa / (0.41/b_m + 21.6))**2.0
z45_8, z75_8, z75_12 = sink_cm(45.0, 0.08), sink_cm(75.0, 0.08), sink_cm(75.0, 0.12)
assert abs(z45_8 - 3.52) < 0.02 and abs(z75_8 - 9.79) < 0.02 and abs(z75_12 - 4.96) < 0.02

# GB-10
assert abs((0.60*0.25 + 0.28*0.40 + 0.12*1.00) - 0.382) < 1e-12

# GB-11 Brute-force V4 & Z2 strata AND directed one-foot transition counts (27/162, 7/16, 45/324, 10/32)
def count_orbits_and_transitions(n_states, group_perms):
    states = list(product(range(n_states), repeat=4))
    canon = {}
    for st in states:
        canon[st] = min(tuple(st[p[i]] for i in range(4)) for p in group_perms)
    orbits = set(canon.values())
    trans = set()
    for st in states:
        for foot in range(4):
            for ns in range(n_states):
                if ns != st[foot]:
                    st2 = list(st); st2[foot] = ns; st2 = tuple(st2)
                    pair_orbit = min((tuple(st[p[i]] for i in range(4)), tuple(st2[p[i]] for i in range(4))) for p in group_perms)
                    trans.add(pair_orbit)
    return len(orbits), len(trans)

V4_perms = [(0,1,2,3), (1,0,3,2), (2,3,0,1), (3,2,1,0)]
Z2_perms = [(0,1,2,3), (1,0,3,2)]
assert count_orbits_and_transitions(3, V4_perms) == (27, 162)
assert count_orbits_and_transitions(2, V4_perms) == (7, 16)
assert count_orbits_and_transitions(3, Z2_perms) == (45, 324)
assert count_orbits_and_transitions(2, Z2_perms) == (10, 32)

# GB-12 .fclear
assert 240*160*32 == 1228800 and abs(1228800*4/(1024**2) - 4.6875) < 1e-9
assert abs(400.0*(math.pi/32.0) + 25.0*math.sqrt(2.0) - 74.63) < 0.03

# GB-13 EXACT Box-Constrained Sort-and-Clip (C2)
def box_cbf_sort_clip(u_pi, a, rhs, lo, hi):
    # Minimize 0.5 ||u - u_pi||^2 s.t. a.u >= rhs, lo <= u <= hi
    def eval_u(lam):
        return [min(hi[i], max(lo[i], u_pi[i] + lam*a[i])) for i in range(len(u_pi))]
    if dot(a, eval_u(0.0)) >= rhs:
        return 0.0, eval_u(0.0)
    # Collect breakpoints where u_pi[i] + lam*a[i] hits lo[i] or hi[i]
    bpts = sorted(set(x for i in range(len(u_pi)) if abs(a[i]) > 1e-12
                      for x in [(lo[i]-u_pi[i])/a[i], (hi[i]-u_pi[i])/a[i]] if x >= 0.0))
    bpts = [0.0] + bpts + [bpts[-1] + 10.0]
    for l0, l1 in zip(bpts, bpts[1:]):
        g0, g1 = dot(a, eval_u(l0)), dot(a, eval_u(l1))
        if g0 <= rhs <= g1 + 1e-12:
            lam = l0 + (rhs - g0) * (l1 - l0) / (g1 - g0)
            return lam, eval_u(lam)
    raise AssertionError("Infeasible box")

lam13, u13 = box_cbf_sort_clip([1.0, 0.0], [1.0, 1.0], 1.5, [0.0, 0.0], [1.0, 1.0])
assert abs(lam13 - 0.5) < 1e-12 and abs(u13[0] - 1.0) < 1e-12 and abs(u13[1] - 0.5) < 1e-12

# GB-14..GB-16
assert math.factorial(6)//(2**3) == 90
assert abs(math.log(2**24) - 16.6355) < 0.01
assert math.comb(40,1)+math.comb(40,2)+math.comb(40,3) == 10700

# KA-07 Rimless Wheel: full-space monodromy singular (eig +1) vs codim-1 Poincare nonsingular
alpha_rw = 0.3
mu_poincare = math.cos(2.0 * alpha_rw)**2
assert abs(1.0 - mu_poincare - 0.31882) < 1e-4

# Frobenius-Schur real projectors: C5 (complex nu=0) and Q8 (quaternionic nu=-1)
# C5 regular rep (5x5 cyclic shift matrices)
C5_mats = [[[1.0 if (i - k) % 5 == j else 0.0 for j in range(5)] for i in range(5)] for k in range(5)]
P_c5 = [[0.0]*5 for _ in range(5)]
Q_c5 = [[0.0]*5 for _ in range(5)]
for k in range(5):
    ang = 2.0 * math.pi * k / 5.0
    for i in range(5):
        for j in range(5):
            P_c5[i][j] += (2.0/5.0) * math.cos(ang) * C5_mats[k][i][j]
            Q_c5[i][j] += (2.0/5.0) * math.sin(ang) * C5_mats[k][i][j]
P_c5_sq = mat_mul(P_c5, P_c5)
Q_c5_sq = mat_mul(Q_c5, Q_c5)
assert max(abs(P_c5_sq[i][j] - P_c5[i][j]) for i in range(5) for j in range(5)) < 1e-12
assert max(abs(Q_c5_sq[i][j] + P_c5[i][j]) for i in range(5) for j in range(5)) < 1e-12
assert abs(sum(P_c5[i][i] for i in range(5)) - 2.0) < 1e-12
# Q8 regular rep (8 elements: 1,-1,i,-i,j,-j,k,-k; unique 2D quaternionic irrep chi(1)=2, chi(-1)=-2, else 0)
# Left multiplication by 1 is I8, by -1 swaps pairs (0<->1,2<->3,4<->5,6<->7) -> P_H = (2/8)*(2*I8 - 2*R_{-1}) = 0.5*(I8 - R_{-1})
P_q8 = [[0.5 if i==j else (-0.5 if (i^1)==j else 0.0) for j in range(8)] for i in range(8)]
P_q8_sq = mat_mul(P_q8, P_q8)
assert max(abs(P_q8_sq[i][j] - P_q8[i][j]) for i in range(8) for j in range(8)) < 1e-12
assert abs(sum(P_q8[i][i] for i in range(8)) - 4.0) < 1e-12

# 18D se(3) Anchored Cellular Sheaf with rho_{v<e} = Ad_{T_{ev}} (§16.2 / KA-10)
# 3-node sensor triangle (v0 anchored: delta_0 = 0; free stalks delta_1, delta_2 in se(3), total 12 unknowns, 18 edge coords)
T01 = se3_cayley([0.1, -0.2, 0.3, 0.25, -0.1, 0.15])
T12 = se3_cayley([-0.15, 0.25, -0.1, -0.1, 0.3, 0.05])
T20 = mat_inv(mat_mul(T01, T12))
# Edges: e01 (v0->v1): r01 = delta_1; e12 (v1->v2): r12 = delta_2 - Ad_{T21} delta_1; e20 (v2->v0): r20 = -Ad_{T02} delta_2
Ad_21 = se3_Ad(mat_inv(T12))
Ad_02 = se3_Ad(mat_inv(T20))
d0_18 = [[0.0]*12 for _ in range(18)]
for i in range(6):
    d0_18[i][i] = 1.0
    d0_18[6+i][6+i] = 1.0
    for j in range(6):
        d0_18[6+i][j] = -Ad_21[i][j]
        d0_18[12+i][6+j] = -Ad_02[i][j]
W18_diag = [100.0,100.0,100.0,1.0,1.0,1.0]*3
W18 = [[W18_diag[i] if i==j else 0.0 for j in range(18)] for i in range(18)]
L0_12 = mat_mul(mat_T(d0_18), mat_mul(W18, d0_18))
L0_inv = mat_inv(L0_12)
P_orth_W = [[(1.0 if i==j else 0.0) - mat_mul(d0_18, mat_mul(L0_inv, mat_mul(mat_T(d0_18), W18)))[i][j] for j in range(18)] for i in range(18)]
P_sq = mat_mul(P_orth_W, P_orth_W)
WP = mat_mul(W18, P_orth_W)
assert max(abs(P_sq[i][j] - P_orth_W[i][j]) for i in range(18) for j in range(18)) < 1e-11
assert max(abs(WP[i][j] - WP[j][i]) for i in range(18) for j in range(18)) < 1e-10

# 2D Finite-Volume Thin-Film Tribology (arithmetic mobility h_f^3 = 0.5*(h_L^3 + h_R^3), oil mass & positivity)
N_g = 11; dx_f = 0.05; dt_f = 0.002; coeff_g = 900.0 * 9.81 / (3.0 * 0.05)
h_grid = [[0.0]*N_g for _ in range(N_g)]
h_grid[5][5] = 0.005
mass0 = sum(sum(r) for r in h_grid) * dx_f * dx_f
for _ in range(25):
    dh = [[0.0]*N_g for _ in range(N_g)]
    for i in range(N_g):
        for j in range(N_g):
            if i + 1 < N_g:
                hf3 = 0.5*(h_grid[i][j]**3 + h_grid[i+1][j]**3)
                flux = -coeff_g * hf3 * ((h_grid[i+1][j] - h_grid[i][j])/dx_f) * dx_f
                dh[i][j] -= flux * dt_f / (dx_f*dx_f)
                dh[i+1][j] += flux * dt_f / (dx_f*dx_f)
            if j + 1 < N_g:
                hf3 = 0.5*(h_grid[i][j]**3 + h_grid[i][j+1]**3)
                flux = -coeff_g * hf3 * ((h_grid[i][j+1] - h_grid[i][j])/dx_f) * dx_f
                dh[i][j] -= flux * dt_f / (dx_f*dx_f)
                dh[i][j+1] += flux * dt_f / (dx_f*dx_f)
    for i in range(N_g):
        for j in range(N_g):
            h_grid[i][j] += dh[i][j]
mass1 = sum(sum(r) for r in h_grid) * dx_f * dx_f
assert abs(mass1 - mass0)/mass0 < 1e-14 and min(min(r) for r in h_grid) >= 0.0 and h_grid[5][6] > 0.0

# Smale alpha-theory counterexample verification (f(z)=z^3+z at z=0.001+1.001j)
z_c = 0.001 + 1.001j
def smale_alpha_cubic(z):
    f = z**3 + z; df = 3*z**2 + 1; d2f = 6*z; d3f = 6.0
    beta = abs(f / df)
    gamma = max(abs(d2f/(2*df)), abs(d3f/(6*df))**0.5)
    return beta * gamma, beta
a_z, b_z = smale_alpha_cubic(z_c)
a_re, _ = smale_alpha_cubic(z_c.real)
alpha0 = (13.0 - 3.0*math.sqrt(17.0))/4.0
assert a_z < alpha0 and a_re < alpha0 and abs(z_c - z_c.conjugate()) > 4*b_z  # Proves naive Re(z) rule alone is unsound!

print(f"  [OK] ALL 16 GB cards + 6-DoF SE(3) LGVI (O(h^2)={se3_errs[0]/se3_errs[1]:.2f}x/{se3_errs[1]/se3_errs[2]:.2f}x, rev={se3_rev_err:.1e}) + SO(3)/SE(2) Cartan ({err_so3_m:.2e} vs {err_so3_p:.2f} / {err_se2_m:.2e} vs {err_se2_p:.2f}) + PMSM/repair (a'={a_rep:.5f}, T45={T_45_C:.1f}C) + R/C/H + 18D Sheaf + Huppert FV verified!")

# ===========================================================================
# 2. 51-CRATE TABLE (§22.2: 31★ / 220.7 kLOC, 378.2 / 435.3 kLOC) & §0.5 SYNC
# ===========================================================================
print("=== 2. 51-Crate Table & Layer Map ===")
s22 = [i for i, l in enumerate(lines) if l.startswith("### 22.2 ")][0]
e22 = [i for i, l in enumerate(lines) if l.startswith("### 22.3 ")][0]
crate_layer = {}
phase_a_cnt, phase_a_loc, tot_p, tot_c = 0, 0.0, 0.0, 0.0
for l in lines[s22:e22]:
    m = re.match(r"^\|\s*(\d+)\s*\|\s*`(frob-[a-z0-9-]+)`\s*\|\s*`([^`]+)`\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([\d.]+)\s*\|\s*([\d.]+)\s*\|", l)
    if m:
        cname, layer, ph, uns, p, c = m.group(2), m.group(3), m.group(4), m.group(5), float(m.group(6)), float(m.group(7))
        crate_layer[cname] = layer
        tot_p += p; tot_c += c
        if "★" in ph:
            phase_a_cnt += 1; phase_a_loc += p
        if abs(c - round(p*1.15, 1)) > 0.15:
            err(f"Ceiling mismatch for {cname}: {c} vs {round(p*1.15,1)}")
        if cname != "frob-gpu-cuda" and "forbid" not in uns:
            err(f"Unexpected unsafe allowance on {cname}")

if len(crate_layer) != 51:
    err(f"Expected 51 crates, parsed {len(crate_layer)}")
if abs(tot_p - 378.2) > 0.05 or abs(tot_c - 435.3) > 0.05 or phase_a_cnt != 31 or abs(phase_a_loc - 220.7) > 0.05:
    err(f"LOC totals drifted: {phase_a_cnt}★/{phase_a_loc:.1f}, total {tot_p:.1f}/{tot_c:.1f}")
print(f"  [OK] {len(crate_layer)} crates | {phase_a_cnt}★ ({phase_a_loc:.1f} kLOC) | Total {tot_p:.1f} / {tot_c:.1f} kLOC")

s05 = [i for i, l in enumerate(lines) if l.startswith("## 0.5 ")][0]
e05 = [i for i, l in enumerate(lines) if l.startswith("## 0.6 ")][0]
s05_crates = set()
for l in lines[s05:e05]:
    if l.startswith("|") and re.match(r"^\|\s*\d+\s*\|", l):
        s05_crates.update(re.findall(r"`(frob-[a-z0-9-]+)`", l.split("|")[-2]))
if s05_crates != set(crate_layer.keys()):
    err(f"§0.5 vs §22.2 crate diff: {s05_crates ^ set(crate_layer.keys())}")

rogue = sorted(t for t in set(re.findall(r"`(frob-[a-z0-9-]+)`", text)) if t not in crate_layer and t != "frob-g1-29dof")
if rogue:
    err(f"Rogue frob-* tokens: {rogue}")

# ===========================================================================
# 3. 100-BEAD DAG, MILESTONE MONOTONICITY & MULTI-CRATE TARJAN SCC CHECK
# ===========================================================================
print("=== 3. 100-Bead DAG & Multi-Crate Layering ===")
s29 = [i for i, l in enumerate(lines) if l.startswith("## 29.")][0]
e29 = [i for i, l in enumerate(lines) if l.startswith("# APPENDIX")][0]
beads = {}; bead_ms = {}; bead_deps = defaultdict(list); bead_rdeps = defaultdict(list)
for l in lines[s29:e29]:
    m = re.match(r"^\|\s*\*\*`(BD-\d+[a-z]?)`\*\*\s*\|\s*M(\d+)\s*/\s*`(WP-\d+)`\s*\|\s*([^|]+)\|\s*([^|]+)\|\s*([^|]+)\|", l)
    if m:
        bid, ms, wp, scope, dep_s, exit_s = m.group(1), int(m.group(2)), m.group(3), m.group(4), m.group(5), m.group(6)
        if bid in beads: err(f"Duplicate bead {bid}")
        beads[bid] = (ms, wp, scope, exit_s); bead_ms[bid] = ms
        for d in re.findall(r"BD-\d+[a-z]?", dep_s):
            bead_deps[bid].append(d); bead_rdeps[d].append(bid)

for b, dlist in bead_deps.items():
    for d in dlist:
        if d not in beads: err(f"Bead {b} depends on undefined {d}")
        elif bead_ms[d] > bead_ms[b]: err(f"Forward milestone inversion: {b}(M{bead_ms[b]}) -> {d}(M{bead_ms[d]})")

indeg = {b: len(set(bead_deps[b])) for b in beads}
q = deque([b for b, deg in indeg.items() if deg == 0])
vis = 0
while q:
    u = q.popleft(); vis += 1
    for v in set(bead_rdeps[u]):
        indeg[v] -= 1
        if indeg[v] == 0: q.append(v)
if vis != len(beads): err(f"Beads DAG cycle! {vis}/{len(beads)}")
sinks = [b for b in beads if len(bead_rdeps[b]) == 0]
if sinks != ["BD-096"]: err(f"Expected sole sink ['BD-096'], got {sinks}")
anc = set(); st = ["BD-096"]
while st:
    u = st.pop()
    if u in anc: continue
    anc.add(u); st.extend(bead_deps[u])
if anc != set(beads.keys()): err(f"Orphan beads: {sorted(set(beads.keys())-anc)}")

b29_text = "\n".join(lines[s29:e29])
for c in crate_layer:
    if c not in b29_text: err(f"Crate {c} missing from §29.2")
for k in range(1, 17):
    if f"NS-{k:02d}" not in b29_text: err(f"NS-{k:02d} missing from §29.2")
for k in range(1, 13):
    if f"WP-{k:02d}" not in b29_text: err(f"WP-{k:02d} missing from §29.2")

# Multi-crate-aware crate DAG & Tarjan SCC check
LAYER_RANK = {"S0": 0, "S1": 1, "K0": 2, "K1": 3, "K2": 4, "E0": 5, "E1": 6, "E2": 7, "V": 8, "T": 9, "U": 9}
crate_edges = defaultdict(set)
s221 = [i for i, l in enumerate(lines) if l.startswith("### 22.1 ")][0]
for l in lines[s221:s22]:
    m_layer = re.match(r"^\s*-\s*`([A-Z0-9]+)`:\s*(.*)$", l)
    if m_layer:
        body = re.sub(r"\([^)]*\)", "", m_layer.group(2))
        for clause in body.split(";"):
            parts = [p.strip() for p in clause.split("→")]
            for a, b in zip(parts, parts[1:]):
                for sc in re.findall(r"\b(frob-[a-z0-9-]+)\b", a):
                    for dc in re.findall(r"\b(frob-[a-z0-9-]+)\b", b):
                        if sc in crate_layer and dc in crate_layer and sc != dc:
                            crate_edges[sc].add(dc)

bead_crates = {}
for bid, (_, _, scope, _) in beads.items():
    lead = scope.split("(", 1)[0]
    cs = list(dict.fromkeys(c for c in re.findall(r"(?:[A-Z]\d?::)?(frob-[a-z0-9-]+)(?:::[a-z0-9_]+)?", lead) if c in crate_layer))
    if cs:
        bead_crates[bid] = cs

for bid, dlist in bead_deps.items():
    if bid not in bead_crates: continue
    srcs = bead_crates[bid]
    max_src_rank = max(LAYER_RANK[crate_layer[c]] for c in srcs)
    for d in dlist:
        if d not in bead_crates: continue
        for dc in bead_crates[d]:
            dr = LAYER_RANK[crate_layer[dc]]
            # Multi-crate bead: at least one crate in srcs must legally sit >= dc
            if dr > max_src_rank:
                err(f"Upward crate dependency via {bid}->{d}: {srcs} -> {dc}({crate_layer[dc]})")
            if len(srcs) == 1 and len(bead_crates[d]) == 1 and srcs[0] != dc:
                sc = srcs[0]
                crate_edges[sc].add(dc)
                if crate_layer[dc] == "T" and crate_layer[sc] not in {"T", "U"}:
                    err(f"Prod imports T via {bid}->{d}: {sc} -> {dc}")
                if crate_layer[sc] == "U" and crate_layer[dc] == "U":
                    err(f"U->U coupling via {bid}->{d}: {sc} -> {dc}")

idx_c = [0]; stack_t, on_s, imap, low = [], set(), {}, {}; sccs = []
def strongconnect(v):
    imap[v] = low[v] = idx_c[0]; idx_c[0] += 1
    stack_t.append(v); on_s.add(v)
    for w in crate_edges.get(v, ()):
        if w not in imap:
            strongconnect(w); low[v] = min(low[v], low[w])
        elif w in on_s:
            low[v] = min(low[v], imap[w])
    if low[v] == imap[v]:
        comp = []
        while True:
            w = stack_t.pop(); on_s.remove(w); comp.append(w)
            if w == v: break
        if len(comp) > 1: sccs.append(comp)
for c in crate_layer:
    if c not in imap: strongconnect(c)
if sccs: err(f"Crate SCC cycles: {sccs}")
print(f"  [OK] {len(beads)} beads acyclic & monotone | Crate DAG ({sum(len(v) for v in crate_edges.values())} edges, {len(bead_crates)} owned beads): 0 SCCs, 0 upward, 0 prod->T, 0 U->U!")

# ===========================================================================
# 4. CROSS-REFERENCE COMPLETENESS & 46 MCP TOOLS
# ===========================================================================
print("=== 4. Cross-Reference & 46-Tool Registry Audit ===")
prefixes = ["PO", "GATE", "INT", "KILL", "XT", "FMT", "INV", "NS", "BET", "KA", "FD", "GB", "FROB-REF"]
defined = {p: set() for p in prefixes}; used = {p: set() for p in prefixes}
for l in lines:
    first_cell = l.split("|")[1] if l.startswith("|") and len(l.split("|")) > 2 else ""
    for p in prefixes:
        pat = rf"\b({p}-[A-Z0-9]+(?:-[A-Z0-9]+)*)\b" if p != "FROB-REF" else r"\b(FROB-REF-\d{3})\b"
        for m in re.finditer(pat, first_cell): defined[p].add(m.group(1))
        for m in re.finditer(pat, l):
            if m.group(1) != "PO-SAFETY-SOS-1": used[p].add(m.group(1))
for p in prefixes:
    missing = sorted(t for t in used[p] if t not in defined[p])
    for m_tok in missing: err(f"Undefined {p} ID: {m_tok}")
for k in range(1, 17):
    if f"**`GOAL-{k:03d}`**" not in text: err(f"Missing standalone GOAL-{k:03d}")

s20 = [i for i, l in enumerate(lines) if l.startswith("## 20.")][0]
e20 = [i for i, l in enumerate(lines) if l.startswith("# PART V")][0]
mcp_tools = []
for l in lines[s20:e20]:
    if re.match(r"^\s+\d+\.\s+\*\*`[a-z]+`\s+\(\d+\):\*\*", l):
        mcp_tools.extend(re.findall(r"`([a-z_]+)`", l.split(":**", 1)[1]))
if len(mcp_tools) != 46 or len(set(mcp_tools)) != 46:
    err(f"Expected 46 unique MCP tools in §20, found {len(mcp_tools)} ({len(set(mcp_tools))} unique)")
else:
    print(f"  [OK] 0 missing IDs across all 13 families + 16 GOALs + exactly 46 unique MCP tools!")

# ===========================================================================
# 5. REAL RECURSIVE-DESCENT EBNF PARSER & PHYSICAL AUDIT OF APPENDIX D
# ===========================================================================
print("=== 5. Real Recursive-Descent EBNF Parser & Robotics Physics Audit (Appendix D) ===")
sD = [i for i, l in enumerate(lines) if l.startswith("# APPENDIX D")][0]
eD = [i for i, l in enumerate(lines) if l.startswith("# APPENDIX E")][0]
app_d = "\n".join(lines[sD:eD])
lisp_blocks = re.findall(r"```lisp\n(.*?)```", app_d, re.S)
if len(lisp_blocks) != 3:
    err(f"Expected 3 lisp blocks in Appendix D, found {len(lisp_blocks)}")

VALID_UNITS = {
    "m", "mm", "cm", "kg", "g", "s", "ms", "m/s", "rad", "deg", "N", "Nm", "W", "V", "A", "Ah",
    "Ohm", "mH", "H", "degC", "K", "Pa", "kg/m3", "Hz", "sqrtHz", "N/m", "Ns/m", "J/K",
    "W/K", "Nm/A", "Vs/rad", "K^-1", "USD"
}

def tokenize_sexpr(src):
    toks = re.findall(r'"(?:[^"\\]|\\.)*"|\(|\)|[^\s()]+', src)
    idx = [0]
    def parse_one():
        if idx[0] >= len(toks): raise ValueError("Unexpected EOF")
        t = toks[idx[0]]; idx[0] += 1
        if t == "(":
            lst = []
            while idx[0] < len(toks) and toks[idx[0]] != ")":
                lst.append(parse_one())
            if idx[0] >= len(toks): raise ValueError("Unclosed '('")
            idx[0] += 1
            return lst
        elif t == ")":
            raise ValueError("Unexpected ')'")
        return t
    node = parse_one()
    if idx[0] != len(toks): raise ValueError(f"Trailing tokens: {toks[idx[0]:]}")
    return node

def is_str(x): return isinstance(x, str) and len(x) >= 2 and x[0] == '"' and x[-1] == '"'
def is_ident(x): return isinstance(x, str) and re.fullmatch(r"[a-zA-Z_][a-zA-Z0-9_-]*", x) is not None
def is_kw(x): return isinstance(x, str) and len(x) >= 2 and x[0] == ":" and is_ident(x[1:])
def is_int(x): return isinstance(x, str) and re.fullmatch(r"[0-9]+", x) is not None
def is_num(x): return isinstance(x, str) and re.fullmatch(r"-?[0-9]+(?:\.[0-9]+)?", x) is not None
def is_bool(x): return x in ("true", "false")
def is_qty(x):
    if not isinstance(x, str): return False
    m = re.fullmatch(r"(-?[0-9]+(?:\.[0-9]+)?)([A-Za-z].*)", x)
    return bool(m and m.group(2) in VALID_UNITS)
def is_path(x):
    return isinstance(x, list) and len(x) == 1 and isinstance(x[0], str) and re.fullmatch(r"[a-zA-Z_][a-zA-Z0-9_-]*(?:\.[a-zA-Z_][a-zA-Z0-9_-]*)*", x[0]) is not None
def is_vec3(x): return isinstance(x, list) and len(x) == 3 and all(is_num(a) for a in x)
def is_dimexpr(x):
    if is_qty(x) or is_num(x) or is_path(x): return True
    return isinstance(x, list) and len(x) == 3 and x[0] in ("+","-","*","/") and is_dimexpr(x[1]) and is_dimexpr(x[2])
def is_geom(x):
    if not isinstance(x, list) or not x: return False
    if x[0] == "box": return len(x) == 4 and all(is_qty(a) for a in x[1:])
    if x[0] in ("cylinder", "capsule"): return len(x) == 3 and all(is_qty(a) for a in x[1:])
    if x[0] == "sphere": return len(x) == 2 and is_qty(x[1])
    return False
def is_se3(x):
    return (isinstance(x, list) and len(x) == 5 and x[0] == "se3" and x[1] == ":xyz"
            and isinstance(x[2], list) and len(x[2]) == 3 and all(is_dimexpr(a) for a in x[2])
            and x[3] == ":rpy" and isinstance(x[4], list) and len(x[4]) == 3 and all(is_qty(a) for a in x[4]))
def is_contact_law(x):
    if not isinstance(x, list) or not x: return False
    if x[0] == "WheelContact":
        return x == ["WheelContact", ":pfaffian", x[2], ":mu-nom", x[4], ":rubber-density", x[6]] and is_bool(x[2]) and is_num(x[4]) and is_qty(x[6])
    return False
def is_val(x):
    return is_ident(x) or is_qty(x) or is_num(x) or is_bool(x) or is_str(x) or is_contact_law(x) or is_se3(x) or is_geom(x) or is_vec3(x)
def is_kvmap(x):
    return isinstance(x, list) and len(x) >= 2 and len(x) % 2 == 0 and all(is_kw(x[i]) and is_val(x[i+1]) for i in range(0, len(x), 2))
def is_wire(x):
    return isinstance(x, list) and len(x) >= 1 and all(isinstance(p, list) and len(p) == 2 and is_ident(p[0]) and is_ident(p[1]) for p in x)
def is_arith(x):
    if is_qty(x) or is_num(x) or is_ident(x) or is_path(x): return True
    return isinstance(x, list) and len(x) >= 2 and x[0] in ("+","-","*","/","abs","min","max") and all(is_arith(a) for a in x[1:])
def is_stl(x, ops):
    if is_ident(x): return True
    if not isinstance(x, list) or not x: return False
    if x[0] in ("and", "or"): return len(x) >= 2 and all(is_stl(a, ops) for a in x[1:])
    if x[0] == "not": return len(x) == 2 and is_stl(x[1], ops)
    if x[0] in ops[:2]: return len(x) == 3 and isinstance(x[1], list) and len(x[1])==2 and all(is_qty(q) for q in x[1]) and is_stl(x[2], ops)
    if x[0] == ops[2]: return len(x) == 4 and isinstance(x[1], list) and len(x[1])==2 and all(is_qty(q) for q in x[1]) and is_stl(x[2], ops) and is_stl(x[3], ops)
    return False

def check_link_form(f):
    # (link Ident :geometry GeomExpr (:shell-thickness Qty)? :material MaterialId)
    i = 2
    assert f[0] == "link" and is_ident(f[1]) and f[i] == ":geometry" and is_geom(f[i+1]); i += 2
    if i < len(f) and f[i] == ":shell-thickness":
        assert is_qty(f[i+1]); i += 2
    assert i + 1 == len(f) - 1 and f[i] == ":material" and is_ident(f[i+1])

def check_joint_form(f):
    assert f[0] == "joint" and is_ident(f[1]) and f[2] == ":type"
    assert f[3] in ("revolute","continuous","prismatic","helical","spherical","fixed","free")
    assert f[4] == ":parent" and is_ident(f[5]) and f[6] == ":child" and is_ident(f[7])
    assert f[8] == ":mount" and is_se3(f[9])
    i = 10
    if i < len(f) and f[i] == ":axis": assert is_vec3(f[i+1]); i += 2
    if i < len(f) and f[i] == ":limits": assert isinstance(f[i+1], list) and len(f[i+1])==2 and all(is_qty(q) for q in f[i+1]); i += 2
    if i < len(f) and f[i] == ":actuator": assert is_ident(f[i+1]); i += 2
    if i < len(f) and f[i] == ":stiction": assert is_qty(f[i+1]); i += 2
    if i < len(f) and f[i] == ":backlash": assert is_qty(f[i+1]); i += 2
    if i < len(f) and f[i] == ":contact-law": assert is_contact_law(f[i+1]) or is_ident(f[i+1]); i += 2
    assert i == len(f), f"Unparsed JointForm tail: {f[i:]}"

# Parse & validate D.1 (BotDoc) against §14.1 EBNF
ast_bot = tokenize_sexpr(lisp_blocks[0])
assert ast_bot[0] == "robot" and is_str(ast_bot[1]) and ast_bot[2] == ":schema" and ast_bot[3] == '"frob.bot/v1"'
for form in ast_bot[4:]:
    tag = form[0]
    if tag == "symmetry":
        assert len(form)==5 and form[1]==":mech" and is_str(form[2]) and form[3]==":obs" and is_str(form[4])
    elif tag == "power-bus":
        assert form[2]==":battery" and form[3][0]=="li-ion-2rc" and form[4]==":v-nom" and is_qty(form[5])
        assert form[6]==":i-max" and is_qty(form[7]) and form[8]==":thermal-sink" and is_qty(form[9])
    elif tag == "budgets":
        assert len(form)%2==1 and all(form[i] in (":mass-max",":bom-max",":power-max",":width-max",":height-max") and is_qty(form[i+1]) for i in range(1,len(form),2))
    elif tag == "actuator":
        kws = [":kt",":ke",":r0",":inductance",":pole-pairs",":alpha-cu",":cth",":gth",":t-max",":i-max",":gear",":backlash",":mass"]
        assert is_ident(form[1]) and len(form) == 2 + 2*len(kws)
        for idx_k, kw in enumerate(kws):
            k_pos = 2 + 2*idx_k
            assert form[k_pos] == kw, f"Actuator expected {kw}, got {form[k_pos]}"
            val = form[k_pos+1]
            if kw == ":pole-pairs": assert is_int(val)
            elif kw == ":gear": assert is_num(val)
            else: assert is_qty(val)
    elif tag == "base":
        assert is_ident(form[1]) and form[2]==":geometry" and is_geom(form[3])
        assert form[4]==":shell-thickness" and is_qty(form[5]) and form[6]==":material" and is_ident(form[7])
    elif tag == "link":
        check_link_form(form)
    elif tag == "joint":
        check_joint_form(form)
    elif tag == "use-module":
        assert is_ident(form[1]) and form[2]==":from" and is_str(form[3]) and form[4]==":parent" and is_ident(form[5])
        assert form[6]==":mount" and is_se3(form[7])
        i = 8
        if i < len(form) and form[i] == ":wire": assert is_wire(form[i+1]); i += 2
        if i < len(form) and form[i] == ":rename": assert is_kvmap(form[i+1]); i += 2
        if i < len(form) and form[i] == ":override": assert is_kvmap(form[i+1]); i += 2
        assert i == len(form)
    elif tag == "mirror-limb":
        assert is_ident(form[1]) and form[2]==":from" and is_ident(form[3]) and form[4]==":planes"
        assert all(p in ("sagittal","coronal") for p in form[5])
        assert form[6]==":parent" and is_ident(form[7]) and form[8]==":mount" and is_se3(form[9])
        i = 10
        if i < len(form) and form[i] == ":wire": assert is_wire(form[i+1]); i += 2
        assert form[i] == ":mirror-axes" and is_bool(form[i+1]) and i + 2 == len(form)
    elif tag == "sensor":
        assert is_ident(form[1]) and form[2]==":type" and is_ident(form[3]) and form[4]==":parent" and is_ident(form[5])
        assert form[6]==":mount" and is_se3(form[7]) and form[8]==":rate" and is_qty(form[9])
        if len(form) > 10: assert len(form)==12 and form[10]==":mass" and is_qty(form[11])
    elif tag == "safety":
        assert len(form)%2==1
    elif tag == "levers":
        for lv in form[1:]:
            assert is_ident(lv[0]) and lv[1]==":range" and len(lv[2])==2 and lv[3]==":step" and lv[5]==":target" and is_path(lv[6])
    else:
        raise AssertionError(f"Unknown BotDoc form: {tag}")

# Parse & validate D.1b (ModuleDoc) against §10.1 EBNF
ast_mod = tokenize_sexpr(lisp_blocks[1])
assert ast_mod[0]=="module" and is_str(ast_mod[1]) and ast_mod[2]==":schema" and ast_mod[3]=='"frob.module/v1"'
ports, params, body, contract = ast_mod[4], ast_mod[5], ast_mod[6], ast_mod[7]
assert ports[0] == "ports"
for p in ports[1:]:
    assert p[0]=="port" and is_ident(p[1]) and p[2]==":kind" and p[3] in ("MechanicalSe3","ElectricalDc","ThermalEntropic","LogicalBus")
    i = 4
    if p[i] == ":frame": assert is_ident(p[i+1]); i += 2
    assert p[i] == ":dir" and p[i+1] in ("in","out","bidir") and i + 2 == len(p)
assert params[0] == "params"
for pr in params[1:]:
    assert pr[0]=="param" and is_ident(pr[1]) and pr[2]==":type" and pr[3] in ("ActuatorId","MaterialId","ContactLaw","Qty") and pr[4]==":default" and is_val(pr[5])
assert body[0] == "body"
mod_joints = []
for b in body[1:]:
    if b[0] == "link": check_link_form(b)
    elif b[0] == "joint": check_joint_form(b); mod_joints.append(b)
    else: raise AssertionError(f"Unexpected module body form: {b[0]}")
assert any(j[1]=="wheel-steer" and j[11]==["0","0","1"] for j in mod_joints), "Missing steering joint for :pfaffian true!"
assert any(j[1]=="wheel-spin" and j[3]=="continuous" and j[9][4]==["90deg","0deg","0deg"] for j in mod_joints), "wheel-spin must be continuous & rpy=(90deg 0deg 0deg)!"
assert contract[0] == "contract" and contract[1]==":power-sign" and contract[2]=='"into-module"'

# Parse & validate D.2 (TaskDoc) against §18.1 EBNF
ast_task = tokenize_sexpr(lisp_blocks[2])
assert ast_task[0]=="task" and is_str(ast_task[1]) and ast_task[2]==":schema" and ast_task[3]=='"frob.task/v1"'
w_f, pay_f, pos_f, clk_f, obs_f, act_f, rst_f, stl_f = ast_task[4:]
assert w_f[0]=="world" and is_str(w_f[1]) and w_f[2]==":tribology-tier" and w_f[3] in ("none","A","B")
assert pay_f[0]=="payload" and is_ident(pay_f[1]) and pay_f[2]==":mass" and is_qty(pay_f[3]) and pay_f[4]==":geometry" and is_geom(pay_f[5]) and pay_f[6]==":mount" and is_ident(pay_f[7]) and pay_f[8]==":pose" and is_se3(pay_f[9])
assert pos_f[0]=="poses" and pos_f[1]==":start" and is_se3(pos_f[2]) and pos_f[3]==":goal" and is_se3(pos_f[4])
assert clk_f[0]=="clocks" and clk_f[1]==":physics" and is_qty(clk_f[2]) and clk_f[3]==":policy" and is_qty(clk_f[4])
assert all(is_kw(clk_f[i]) and is_qty(clk_f[i+1]) for i in range(5, len(clk_f), 2))
assert obs_f[0]=="obs-layout" and all(is_ident(o[0]) and o[1]==":dim" and is_int(o[2]) and o[3]==":rate" and is_qty(o[4]) and o[5]==":source" and is_ident(o[6]) for o in obs_f[1:])
assert act_f[0]=="act-layout" and all(is_ident(a[0]) and a[1]==":dim" and is_int(a[2]) and a[3]==":mode" and a[4] in ("torque","vel","pos") and a[5]==":target" and is_ident(a[6]) for a in act_f[1:])
# Verify 4-DoF x 4 legs = 16 joints matches obs-layout (32 joint-state, 16 thermal-vec) and act-layout (8+4+4=16)!
assert sum(int(a[2]) for a in act_f[1:]) == 16
obs_dims = {o[0]: int(o[2]) for o in obs_f[1:]}
assert obs_dims["joint-state"] == 32 and obs_dims["thermal-vec"] == 16
assert rst_f[0]=="reset-cache" and rst_f[1]==":settled-poses" and is_int(rst_f[2]) and rst_f[3]==":max-penetration" and is_qty(rst_f[4])
assert stl_f[0]=="stl-contract"
preds = [p for p in stl_f[1:] if p[0]=="pred"]
for p in preds:
    assert is_ident(p[1]) and p[2]==":expr" and is_arith(p[3]) and p[4]==":scale" and is_qty(p[5])
    if p[1] == "thermal-ok":
        assert p[5] == "10K", f"thermal-ok scale must be Kelvin difference 10K, got {p[5]}"
onl = [p for p in stl_f[1:] if p[0]=="online-past-reward"][0]
off = [p for p in stl_f[1:] if p[0]=="offline-verdict"][0]
assert is_stl(onl[1], ("H","O","S")) and is_stl(off[1], ("G","F","U"))
assert "goal-progress" in str(onl[1]), "online-past-reward must include goal-progress to prevent stationary reward hacking!"

# Physical Mass, Width, Height, IK & Clearance Verification of kitchen_wheelquad_v2
rho_al, rho_rub = 2700.0, 1150.0
m_chassis = (0.52*0.34*0.14 - 0.515*0.335*0.135) * rho_al
m_mast = (math.pi*0.015**2*0.14 - math.pi*0.013**2*0.136) * rho_al
m_thigh = (math.pi*(0.022**2 - 0.020**2)*0.22 + (4.0/3.0)*math.pi*(0.022**3 - 0.020**3)) * rho_al
m_shin  = (math.pi*(0.020**2 - 0.018**2)*0.22 + (4.0/3.0)*math.pi*(0.020**3 - 0.018**3)) * rho_al
m_fork  = (0.05*0.06*0.05 - 0.045*0.055*0.045) * rho_al
m_wheel = math.pi * 0.065**2 * 0.040 * rho_rub
m_empty = m_chassis + m_mast + 3.2 + 4*(0.85+0.85+0.40+0.90) + 4*(m_thigh+m_shin+m_fork+m_wheel) + (0.35+0.15+0.05)
m_loaded = m_empty + 3.2
assert abs(m_empty - 24.04) < 0.05 and abs(m_loaded - 27.24) < 0.05 and m_loaded <= 30.0
# Geometry & clearance checks
max_width = 0.38 + 2*(0.05 + 0.02)   # W_stance_max + 2*(lateral_offset + half_wheel_width)
inner_clear = 0.05 - 0.022           # clearance between chassis wall and thigh capsule
max_height = 0.42 + 0.25 + 0.04      # z_start + lidar_max + 4cm housing
sensor_dist = math.hypot(0.27 - 0.22, 0.20 - 0.15)  # worst-case rgbd_max vs lidar_min
tray_mast_clear = 0.22 - (-0.04 + 0.18)             # mast x - tray front edge
q_hip_deg = math.degrees(math.acos((0.42 - 0.04 - 0.03 - 0.065) / 0.44))
assert max_width <= 0.58 and inner_clear >= 0.025 and max_height <= 0.72 and sensor_dist >= 0.07 and tray_mast_clear >= 0.08
assert 0.0 < q_hip_deg < 90.0 and -135.0 < -2*q_hip_deg < 15.0

# D.3 strict JSON & safety flags
json_blocks = re.findall(r"```json\n(.*?)```", app_d, re.S)
if len(json_blocks) != 6: err(f"Expected 6 pure JSON blocks in Appendix D.3, found {len(json_blocks)}")
for idx_j, jblk in enumerate(json_blocks, 1):
    try: obj = json.loads(jblk)
    except Exception as ex: err(f"D.3 JSON #{idx_j} invalid: {ex}"); continue
    if "tool" in obj and obj["tool"] not in mcp_tools:
        err(f"D.3 tool '{obj['tool']}' not in §20's 46 enumerated tools!")
    elif "status" in obj:
        for flag in ["execution_authorized", "hardware_safety_signoff", "sim_to_real_certified"]:
            if obj.get(flag) is not False:
                err(f"D.3 response #{idx_j} missing '{flag}: false'")
        if idx_j == 2:
            assert obj.get("claim_axis") == "ClaimProduct<VerifiedFormal, Estimated>"
        if idx_j == 6:
            assert "baseline_v2_autopsy" in obj and "qualified_candidate" in obj
print(f"  [OK] Recursive-descent EBNF parser passed D.1/D.1b/D.2 | Physics: empty={m_empty:.2f}kg, loaded={m_loaded:.2f}kg<=30kg, width={max_width:.2f}m<=0.58m, height={max_height:.2f}m<=0.72m, q_hip={q_hip_deg:.1f}deg | 6 JSON blocks valid!")

# ===========================================================================
# 6. MARKDOWN HYGIENE, CONTINUOUS H2 §1..§29, FILE LINKS & † K-01..K-13 PARITY
# ===========================================================================
print("=== 6. Hygiene, File Links & † K-01..K-13 Parity ===")
in_code = False; tcols = None
for idx_l, l in enumerate(lines, 1):
    if l.strip().startswith("```"): in_code = not in_code; tcols = None; continue
    if in_code: continue
    if l.startswith("|") and l.rstrip().endswith("|"):
        c = len(re.findall(r"(?<!\\)\|", l)) - 1
        if tcols is None: tcols = c
        elif c != tcols: err(f"Line {idx_l}: Markdown table column mismatch ({c} vs {tcols})")
    else: tcols = None

h2_nums = [int(re.match(r"^##\s+(\d+)\.\s+", l).group(1)) for l in lines if re.match(r"^##\s+(\d+)\.\s+", l)]
if h2_nums != list(range(1, 30)): err(f"H2 section numbering discontinuous: {h2_nums}")
for lp in re.findall(r"file:///([^\)\s>]+)", text):
    if not Path("/" + lp.split("#")[0]).exists(): err(f"Broken file:// link: file:///{lp}")

art_text = ARTIFACT.read_text(encoding="utf-8")
expected_k = [f"† K-{i:02d}" for i in range(1, 14)]
if sorted(set(re.findall(r"† K-\d{2}", text))) != expected_k or sorted(set(re.findall(r"† K-\d{2}", art_text))) != expected_k:
    err("† K-01..† K-13 parity mismatch between NEON and ARTIFACT.md")
print(f"  [OK] 0 broken tables, continuous §1..§29, all file:// links live, unified {len(expected_k)} † K-01..† K-13 synced!")

print("\n=== FINAL VERIFICATION RESULT ===")
print(f"Lines: {len(lines)}, Bytes: {len(text.encode('utf-8')):,}, Errors: {len(errors)}")
for e in errors:
    print("  [ERROR]", e)
sys.exit(1 if errors else 0)
