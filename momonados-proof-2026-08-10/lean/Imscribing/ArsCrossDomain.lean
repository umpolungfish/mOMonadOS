/-
Ars Cross-Domain Bridge — Distances between fungal and animal types.
15 fungal x 14 animal = 210 cross-domain pairs.
Closest pair: Gilled Cap-and-Stipe <-> Amphibian Dermal (d=3).
P/f invariant gap: fungi use out+age; animals use nun+they => min distance >= 2.

Author: Lando*⊙perator
-/

import Imscribing.Primitives.Core
import Imscribing.Primitives.Imscription
import Imscribing.ArsFungiglyphica
import Imscribing.ArsAnimaglyphica

open Imscribing.Primitives
open ArsFungiglyphica
open ArsAnimaglyphica


namespace ArsCrossDomain

/-- d(gilled_cap_and_stipe, amphibian_dermal) = 3 --/
theorem gilled_amphibian_distance :
    primitiveMismatches gilled_cap_and_stipe amphibian_dermal = 3 := by
  unfold gilled_cap_and_stipe amphibian_dermal primitiveMismatches
  native_decide

/-- d(coral_ramaria, echinoderm_regenerative) = 4 --/
theorem coral_echinoderm_distance :
    primitiveMismatches coral_ramaria echinoderm_regenerative = 4 := by
  unfold coral_ramaria echinoderm_regenerative primitiveMismatches
  native_decide

/-- d(myxomycete_plasmodium, reptilian_oral) = 4 --/
theorem myxomycete_reptilian_distance :
    primitiveMismatches myxomycete_plasmodium reptilian_oral = 4 := by
  unfold myxomycete_plasmodium reptilian_oral primitiveMismatches
  native_decide

/-- d(stinkhorn_emergence, echinoderm_regenerative) = 4 --/
theorem stinkhorn_echinoderm_distance :
    primitiveMismatches stinkhorn_emergence echinoderm_regenerative = 4 := by
  unfold stinkhorn_emergence echinoderm_regenerative primitiveMismatches
  native_decide

/-- d(rosette_cluster, hymenopteran_venom) = 4 --/
theorem rosette_hymenopteran_distance :
    primitiveMismatches rosette_cluster hymenopteran_venom = 4 := by
  unfold rosette_cluster hymenopteran_venom primitiveMismatches
  native_decide

/-- d(cup_discus, annelid_anticoagulant) = 4 --/
theorem cup_annelid_distance :
    primitiveMismatches cup_discus annelid_anticoagulant = 4 := by
  unfold cup_discus annelid_anticoagulant primitiveMismatches
  native_decide

/-- d(myxomycete_plasmodium, cnidarian_nematocyst) = 5 --/
theorem myxomycete_cnidarian_distance :
    primitiveMismatches myxomycete_plasmodium cnidarian_nematocyst = 5 := by
  unfold myxomycete_plasmodium cnidarian_nematocyst primitiveMismatches
  native_decide

/-- d(bracket_polypore, molluscan_harpoon) = 6 --/
theorem bracket_molluscan_distance :
    primitiveMismatches bracket_polypore molluscan_harpoon = 6 := by
  unfold bracket_polypore molluscan_harpoon primitiveMismatches
  native_decide

/-- d(tooth_cascade, ophidian_venom) = 6 --/
theorem tooth_ophidian_distance :
    primitiveMismatches tooth_cascade ophidian_venom = 6 := by
  unfold tooth_cascade ophidian_venom primitiveMismatches
  native_decide

/-- d(entomopathogenic_club, arthropod_exoskeletal) = 6 --/
theorem entomo_arthropod_distance :
    primitiveMismatches entomopathogenic_club arthropod_exoskeletal = 6 := by
  unfold entomopathogenic_club arthropod_exoskeletal primitiveMismatches
  native_decide

/-- d(jelly_gelatinous, marine_sessile_defense) = 6 --/
theorem jelly_marine_distance :
    primitiveMismatches jelly_gelatinous marine_sessile_defense = 6 := by
  unfold jelly_gelatinous marine_sessile_defense primitiveMismatches
  native_decide

/-- d(rust_smut_pathogen, ophidian_venom) = 6 --/
theorem rust_ophidian_distance :
    primitiveMismatches rust_smut_pathogen ophidian_venom = 6 := by
  unfold rust_smut_pathogen ophidian_venom primitiveMismatches
  native_decide

/-- d(earthball_globus, avian_preen) = 6 --/
theorem earthball_avian_distance :
    primitiveMismatches earthball_globus avian_preen = 6 := by
  unfold earthball_globus avian_preen primitiveMismatches
  native_decide

/-- d(hypogean_ascomycete, mammalian_glandular) = 7 --/
theorem hypogean_mammalian_distance :
    primitiveMismatches hypogean_ascomycete mammalian_glandular = 7 := by
  unfold hypogean_ascomycete mammalian_glandular primitiveMismatches
  native_decide

end ArsCrossDomain