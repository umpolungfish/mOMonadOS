
// IMASM token definitions
const TOKENS = ['VINIT','TANCH','AFWD','AREV','CLINK','IMSCRIB','FSPLIT','FFUSE','EVALT','EVALF','ENGAGR','IFIX'];
const GLYPH = {'VINIT':'\u22a2','TANCH':'\u22a3','AFWD':'>','AREV':'<','CLINK':'=','IMSCRIB':'\u2299',
  'FSPLIT':'\u25c7','FFUSE':'\u25cf','EVALT':'+','EVALF':'×','ENGAGR':'\u229e','IFIX':'\u00ac'};
const FAMILY = {'VINIT':0,'TANCH':0,'AFWD':0,'AREV':0,'CLINK':0,'IMSCRIB':0,'FSPLIT':1,'FFUSE':1,'EVALT':2,'EVALF':2,'ENGAGR':2,'IFIX':3};
const FAM_NAMES = ['LOGICAL','FROBENIUS','DIALETHEIA','LINEAR'];
const FAM_COLORS = ['#6af','#a84bb8','#e8b84b','#6f6'];

// Canonical IMASM programs
const CANONICALS = {
  'I_Dialetheic_Bootstrap': ['IMSCRIB','EVALT','FSPLIT','EVALF','FFUSE','ENGAGR','IFIX','IMSCRIB'],
  'II_Void_Genesis': ['VINIT','TANCH','AFWD','FSPLIT','CLINK','FFUSE','IFIX','IMSCRIB'],
  'III_Anchor_Protocol': ['TANCH','AREV','VINIT','AFWD','TANCH','CLINK','IFIX','IMSCRIB'],
  'IV_Dual_Bootstrap': ['IMSCRIB','AFWD','FFUSE','FSPLIT','AREV','CLINK','IFIX','IMSCRIB'],
  'V_Linear_Chain': ['IFIX','IFIX','IFIX','IFIX','IFIX','IFIX','IFIX','IFIX'],
  'VI_Empty_Bootstrap': ['VINIT','IMSCRIB','VINIT','IMSCRIB','VINIT','IMSCRIB','VINIT','IMSCRIB'],
  'VII_Parakernel': ['EVALF','AREV','FSPLIT','EVALT','AFWD','FFUSE','ENGAGR','IFIX'],
  'VIII_Frobenius_Kernel': ['VINIT','FSPLIT','FFUSE','TANCH'],
  'IX_Chiral_Pairs': ['AFWD','AREV','AFWD','AREV','AFWD','AREV','AFWD','AREV'],
  'X_Truth_Machine': ['IMSCRIB','FSPLIT','EVALT','IFIX','IMSCRIB','FSPLIT','EVALF','IFIX'],
  'XI_Eternal_Return': ['IMSCRIB','AFWD','AREV','IMSCRIB','AFWD','AREV','IMSCRIB','AFWD'],
  'XII_ROM_Burn': ['EVALT','IFIX','EVALF','IFIX','ENGAGR','IFIX','IMSCRIB','IFIX'],
  'agent_loop': ['VINIT','IMSCRIB','FSPLIT','EVALT','CLINK','FFUSE','IFIX','ENGAGR','AREV','CLINK','TANCH'],
  'bootstrap_loop': ['IMSCRIB','AREV','FSPLIT','AFWD','FFUSE','CLINK','IFIX','IMSCRIB'],
};
