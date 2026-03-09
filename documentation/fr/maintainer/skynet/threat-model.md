# Modèle de menace du protocole Skynet

## Aperçu

Ce document utilise le cadre de modèle de menace STRIDE pour analyser les principales menaces de sécurité auxquelles est confronté le protocole Skynet et les mesures d'atténuation correspondantes.

### Principes de conception

Le protocole Skynet suit les principes de conception clés suivants :

1. **Priorité à l'efficacité** : Dans le cadre d'une sécurité de base garantie, prioriser les performances du système et l'expérience utilisateur
2. **Solution moderne** : Adopter des primitives et protocoles cryptographiques modernes entièrement vérifiés
3. **Conception extensible** : La conception architecturale prend en charge l'extension fluide des mécanismes de sécurité futurs
4. **Sécurité pratique** : Fournir une garantie de sécurité de niveau entreprise, ne pas poursuivre une conception excessive de niveau militaire

### Choix des paramètres de sécurité (priorité à l'efficacité)

| Paramètre | Valeur | Description |
|------|------|------|
| **Algorithme de signature** | Ed25519 | Signature de courbe elliptique efficace et sécurisée |
| **Échange de clés** | X25519 | Échange de clés Diffie-Hellman efficace |
| **Algorithme de hachage** | Blake3-256 | Hachage cryptographique hautes performances |
| **Chiffrement symétrique** | ChaCha20-Poly1305 | Chiffrement authentifié, implémentation logicielle efficace |
| **Protocole Noise** | Noise_XX_25519_ChaChaPoly_BLAKE2s | Protocole de poignée de main léger |
| **Nombre de pré-clés X3DH** | 100 | Équilibrer sécurité et stockage |
| **Suite cryptographique MLS** | MLS10_128_HPKEX25519_AES128GCM_SHA256_Ed25519 | Suite MLS standard |

### Réservation de capacité d'extension

La conception du protocole réserve les points d'extension de sécurité suivants :
- Prise en charge de la protection des métadonnées de réseau mixte (Loopix, etc.)
- Prise en charge de la migration des algorithmes de cryptographie post-quantique (PQC)
- Prise en charge d'algorithmes de consensus enfichables
- Prise en charge d'un moteur de stratégie d'autorisation personnalisé

## Cadre de modèle de menace : STRIDE

| Type de menace | Description |
|---------|------|
| **S**poofing (Usurpation) | Usurper l'identité d'autrui |
| **T**ampering (Altération) | Altérer des données ou du code |
| **R**épudiation (Répudiation) | Nier avoir effectué une opération |
| **I**nformation Disclosure (Divulgation d'informations) | Divulguer des informations sensibles |
| **D**enial of Service (Déni de service) | Rendre un système ou un service indisponible |
| **E**lévation of Privilege (Élévation de privilèges) | Obtenir des privilèges non autorisés |

---

## 1. Usurpation (Spoofing)

### 1.1 Usurpation d'identité de nœud
**Menace** : Un attaquant usurpe l'identité d'un nœud de service légitime pour rejoindre le réseau et inciter les clients à se connecter.

**Mesures d'atténuation** :
- Chaque nœud possède une paire de clés Ed25519, NodeID = blake3(public_key)
- La communication entre les nœuds utilise le protocole Noise (mode XX), vérifiant mutuellement le NodeID
- Le client peut préconfigurer une liste de nœuds de confiance de départ

### 1.2 Usurpation d'identité d'utilisateur
**Menace** : Un attaquant usurpe l'identité d'un autre utilisateur pour envoyer des messages ou effectuer des opérations.

**Mesures d'atténuation** :
- Toutes les opérations utilisateur nécessitent une signature avec la clé privée Ed25519
- Les nœuds de service vérifient la signature avant d'accepter l'opération
- Le chiffrement de bout en bout garantit que seul l'utilisateur possédant la clé privée correspondante peut déchiffrer le message

---

## 2. Altération (Tampering)

### 2.1 Altération de données
**Menace** : Un nœud de service malveillant altère les données du sous-réseau (liste des membres, historique des messages, etc.).

**Mesures d'atténuation** :
- Toutes les données importantes sont signées par l'initiateur
- Le client récupère les données depuis plusieurs nœuds de maintenance et effectue une vérification croisée
- Journal d'opérations immuable + instantané de hachage, permettant de détecter l'altération historique
- En cas de conflit, adopter le principe de majorité

### 2.2 Altération de message
**Menace** : Un attaquant altère le contenu du message pendant la transmission.

**Mesures d'atténuation** :
- Le chiffrement de bout en bout utilise ChaCha20-Poly1305, fournissant un chiffrement authentifié
- Le message contient content_hash (Blake3), permettant de vérifier l'intégrité
- Le destinataire vérifie la balise Poly1305, rejette le message en cas d'échec

---

## 3. Répudiation (Repudiation)

### 3.1 Répudiation d'opération
**Menace** : Un utilisateur nie avoir envoyé un certain message ou effectué une certaine opération.

**Mesures d'atténuation** :
- Toutes les opérations sont accompagnées d'une signature Ed25519
- Journal d'opérations immuable enregistré de manière permanente (optionnel)
- La signature peut être vérifiée par un tiers

---

## 4. Divulgation d'informations (Information Disclosure)

### 4.1 Divulgation de contenu
**Menace** : Un nœud de service ou un homme du milieu obtient le contenu en clair du message.

**Mesures d'atténuation** :
- Tous les messages sont chiffrés de bout en bout (X3DH + Double Ratchet pour les conversations privées, MLS pour les conversations de groupe)
- Les nœuds de service ne stockent et ne transfèrent que du texte chiffré
- Les clés sont gérées par le client, le protocole n'implique pas le stockage des clés

### 4.2 Divulgation de métadonnées
**Menace** : Les nœuds de service peuvent voir les relations de communication (qui discute avec qui, quand).

**Mesures d'atténuation** :
- Isolation des sous-réseaux : Les données de différents sous-réseaux sont complètement isolées
- **Réservation extensible** : L'architecture du protocole prend en charge l'introduction future d'un réseau mixte (comme Loopix) pour masquer les métadonnées (pas une exigence obligatoire pour la version actuelle)

### 4.3 Divulgation d'auth_id
**Menace** : auth_id est divulgué dans le sous-réseau, associant l'identité de l'utilisateur entre les sous-réseaux.

**Mesures d'atténuation** :
- auth_id n'est utilisé que pour l'authentification initiale, jamais stocké dans les données du sous-réseau
- L'intérieur du sous-réseau n'utilise que user_id
- La spécification du protocole interdit explicitement la propagation d'auth_id dans le sous-réseau

---

## 5. Déni de service (DoS)

### 5.1 DoS de nœud de service
**Menace** : Un attaquant consomme les ressources d'un nœud de service, le rendant incapable de traiter les requêtes légitimes.

**Mesures d'atténuation** :
- Architecture de réseau pair à pair, le client peut basculer vers d'autres nœuds
- Limitation de débit : La fréquence des requêtes de chaque client est limitée
- Décentralisation : Pas de point de défaillance unique

### 5.2 DoS de sous-réseau
**Menace** : Un grand nombre de nœuds malveillants deviennent des nœuds de maintenance du sous-réseau, refusant le service.

**Mesures d'atténuation** :
- Sous-réseaux fermés : L'adhésion nécessite une vérification par l'administrateur
- Sous-réseaux ouverts optionnels : Introduction d'une preuve de travail ou d'un mécanisme de dépôt
- Le client peut lire depuis plusieurs nœuds, adopter la majorité

---

## 6. Élévation de privilèges (Elevation of Privilege)

### 6.1 Élévation de privilèges de membre ordinaire
**Menace** : Un membre ordinaire obtient les privilèges d'administrateur.

**Mesures d'atténuation** :
- Les opérations de privilège nécessitent une signature de l'administrateur
- Vérification multi-nœuds : Nécessite la confirmation de la majorité des nœuds de maintenance
- Le journal d'opérations enregistre toutes les modifications de privilèges

### 6.2 Dépassement de privilèges de nœud de service
**Menace** : Un nœud de service effectue des opérations non autorisées.

**Mesures d'atténuation** :
- Architecture à confiance zéro : Ne suppose pas que les nœuds de service sont fiables
- Toutes les opérations critiques nécessitent une signature du client
- Les nœuds de service ne vérifient que les signatures et transfèrent les messages

---

## Résumé

Le protocole Skynet répond à divers types de menaces via les mécanismes clés suivants :
1. **Signatures cryptographiques** : Prévenir l'usurpation, l'altération et la répudiation
2. **Chiffrement de bout en bout** : Prévenir la divulgation de contenu
3. **Vérification multi-nœuds** : Prévenir les méfaits d'un seul nœud
4. **Architecture à confiance zéro** : Ne faire confiance à aucune partie prenante
5. **Isolation des sous-réseaux** : Limiter la portée de l'impact de l'attaque

### Positionnement de sécurité

L'objectif de sécurité du protocole Skynet est **une sécurité pratique de niveau entreprise**, plutôt qu'une conception excessive de niveau militaire :
- ✅ Protéger la vie privée des communications d'entreprise et la sécurité des données
- ✅ Résister aux attaques réseau courantes et aux menaces internes
- ✅ Fournir de bonnes performances et une bonne expérience utilisateur
- ❌ Ne pas poursuivre la résistance à une attaque à grande échelle d'un adversaire de niveau national
- ❌ Ne pas sacrifier la disponibilité pour une sécurité théorique absolue

### Directions d'extension futures

La conception du protocole réserve de l'espace d'extension pour les mises à niveau de sécurité futures :
1. **Protection des métadonnées** : Peut introduire une technologie de réseau mixte pour masquer les relations de communication
2. **Migration post-quantique** : Prendre en charge une transition fluide vers les algorithmes de cryptographie post-quantique
3. **Renforcement du consensus** : Peut mettre à niveau l'algorithme de consensus et le seuil de sécurité selon les besoins
4. **Renforcement de l'audit** : Peut étendre des fonctions d'audit de sécurité et de conformité plus fines
