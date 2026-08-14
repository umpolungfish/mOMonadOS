-- Imscribing/PowerMath/ShavianDefs.lean
-- ASCII-named Imscription constants mapping Shavian glyph semantics.
-- Each glyph represents a full Imscription where exactly one primitive field
-- is set to its characteristic value and all others are at the minimum.
-- Used by PowerMath theorem scaffolds (the IGProtocol arrow chains).
--
-- Author: Lando⊗⊙perator

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription

namespace Imscribing.PowerMath
open Primitives
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection
     Stoichiometry Chirality

-- ── Default (minimum) Imscription ────────────────────────────────────────────

def defaultImscription : Imscription := {
  dim  := dead
  top  := judge
  rel  := ado
  pol  := church
  fid  := age
  kin  := yea
  gran := bib
  gram := vow
  crit := woe
  chir := fee
  stoi := hung
  prot := awe
}

-- ── Universal Imscription (O_∞ tier: the grammar's own type) ─────────────────

def universalImscription : Imscription := {
  dim  := if'
  top  := are
  rel  := ian
  pol  := or'
  fid  := peep
  kin  := egg
  gran := ice
  gram := measure
  crit := roar
  chir := wool
  stoi := up
  prot := ah
}

-- ── Dimensionality glyphs ───────────────────────────────────────────────────

def dim_dead  : Imscription := { defaultImscription with dim := dead }
def dim_ash   : Imscription := { defaultImscription with dim := ash }
def dim_array : Imscription := { defaultImscription with dim := array }
def dim_if    : Imscription := { defaultImscription with dim := if' }

-- ── Topology glyphs ─────────────────────────────────────────────────────────

def top_judge : Imscription := { defaultImscription with top := judge }
def top_eat   : Imscription := { defaultImscription with top := eat }
def top_mime  : Imscription := { defaultImscription with top := mime }
def top_oil   : Imscription := { defaultImscription with top := oil }
def top_are   : Imscription := { defaultImscription with top := are }

-- ── Relational glyphs ───────────────────────────────────────────────────────

def rel_ado : Imscription := { defaultImscription with rel := ado }
def rel_tot : Imscription := { defaultImscription with rel := tot }
def rel_ear : Imscription := { defaultImscription with rel := ear }
def rel_ian : Imscription := { defaultImscription with rel := ian }

-- ── Polarity glyphs ───────────────────────────────────────────────────────

def pol_church : Imscription := { defaultImscription with pol := church }
def pol_yew    : Imscription := { defaultImscription with pol := yew }
def pol_out    : Imscription := { defaultImscription with pol := out }
def pol_nun    : Imscription := { defaultImscription with pol := nun }
def pol_or     : Imscription := { defaultImscription with pol := or' }

-- ── Fidelity glyphs ───────────────────────────────────────────────────────

def fid_age  : Imscription := { defaultImscription with fid := age }
def fid_they : Imscription := { defaultImscription with fid := they }
def fid_peep : Imscription := { defaultImscription with fid := peep }

-- ── KineticChar glyphs ────────────────────────────────────────────────────

def kin_yea  : Imscription := { defaultImscription with kin := yea }
def kin_loll : Imscription := { defaultImscription with kin := loll }
def kin_egg  : Imscription := { defaultImscription with kin := egg }
def kin_on   : Imscription := { defaultImscription with kin := on }
def kin_air  : Imscription := { defaultImscription with kin := air }

-- ── Granularity glyphs ────────────────────────────────────────────────────

def gran_bib   : Imscription := { defaultImscription with gran := bib }
def gran_thigh : Imscription := { defaultImscription with gran := thigh }
def gran_ice   : Imscription := { defaultImscription with gran := ice }

-- ── Grammar (Composition) glyphs ───────────────────────────────────────────

def gram_vow     : Imscription := { defaultImscription with gram := vow }
def gram_gag     : Imscription := { defaultImscription with gram := gag }
def gram_measure : Imscription := { defaultImscription with gram := measure }
def gram_ooze    : Imscription := { defaultImscription with gram := ooze }

-- ── Criticality glyphs ────────────────────────────────────────────────────

def crit_woe   : Imscription := { defaultImscription with crit := woe }
def crit_monad : Imscription := { defaultImscription with crit := monad }
def crit_roar  : Imscription := { defaultImscription with crit := roar }
def crit_err   : Imscription := { defaultImscription with crit := err }
def crit_haha  : Imscription := { defaultImscription with crit := haha }

-- ── Chirality glyphs ───────────────────────────────────────────────────────

def chir_fee  : Imscription := { defaultImscription with chir := fee }
def chir_kick : Imscription := { defaultImscription with chir := kick }
def chir_sure : Imscription := { defaultImscription with chir := sure }
def chir_wool : Imscription := { defaultImscription with chir := wool }

-- ── Stoichiometry glyphs ───────────────────────────────────────────────────

def stoi_hung : Imscription := { defaultImscription with stoi := hung }
def stoi_so   : Imscription := { defaultImscription with stoi := so }
def stoi_up   : Imscription := { defaultImscription with stoi := up }

-- ── Protection glyphs ──────────────────────────────────────────────────────

def prot_awe : Imscription := { defaultImscription with prot := awe }
def prot_oak : Imscription := { defaultImscription with prot := oak }
def prot_ah  : Imscription := { defaultImscription with prot := ah }
def prot_zoo : Imscription := { defaultImscription with prot := zoo }

end Imscribing.PowerMath
