# Modèle de sécurité

Ce document décrit comment AI Company, en tant qu'implémentation de sous-réseau sur le protocole Skynet, complète les mécanismes de sécurité au niveau de l'architecture.

## Vue d'ensemble

### Division des responsabilités par couche

| Couche | Responsabilités | Description |
|--------|-----------------|-------------|
| **Couche protocole Skynet** | Communication réseau, primitives cryptographiques de base, abstraction de sous-réseau | Légère, flexible, universelle |
| **Couche implémentation sous-réseau (AI Company)** | Mécanismes de sécurité spécifiques au scénario, guide d'implémentation client | Implémenté à la demande selon le type de sous-réseau |

Le protocole Skynet définit uniquement les mécanismes de sécurité **minimaux nécessaires** pour rester léger et flexible. Les sous-réseaux spécifiques (comme AI Company) complètent avec des mesures de sécurité supplémentaires au niveau de l'architecture selon leurs propres besoins.

---

## I. Protection des métadonnées

### Positionnement couche protocole Skynet
Le protocole Skynet **ne fournit pas** de protection des métadonnées, préservant ainsi la légèreté du protocole.

### Implémentation par la couche sous-réseau AI Company

Pour les applications d'entreprise, le schéma de protection des métadonnées est conçu et implémenté par AI Company :

#### Mesures de base (recommandées)
| Mesure | Description |
|--------|-------------|
| **Randomisation du routage** | Le client sélectionne aléatoirement les nœuds de service |
| **Réduction de la précision des horodatages** | Précision à la seconde (plutôt qu'à la milliseconde) |

#### Planification à long terme : Réseau mixte
Si une protection forte des métadonnées est nécessaire, AI Company pourra introduire un réseau mixte (comme Loopix) dans les versions futures, mais cela nécessitera :
- Des modifications architecturales importantes
- Une acceptation d'une latence et d'une surcharge de bande passante plus élevées
- Une utilisation uniquement pour les scénarios à exigences de confidentialité extrêmes

---

## II. Migration vers la cryptographie post-quantique (PQC)

### Positionnement couche protocole Skynet
Le protocole Skynet **réserve des points d'extension** pour prendre en charge la migration vers les algorithmes PQC, mais ne l'impose pas.

### Implémentation par la couche sous-réseau AI Company

#### 2.1 Mode certificat mixte
AI Company est responsable de l'implémentation supportant à la fois la cryptographie classique et la cryptographie post-quantique :
- **Paires de clés doubles** : Chaque identité détient simultanément une paire de clés Ed25519 et CRYSTALS-Dilithium
- **Signatures doubles** : Les opérations importantes sont signées avec les deux algorithmes simultanément
- **Migration progressive** : Prise en charge de la vérification d'abord, puis passage progressif à PQC comme principal

#### 2.2 Suites cryptographiques PQC
| Composant | Algorithme classique | Algorithme PQC | Mode mixte |
|-----------|---------------------|---------------|-----------|
| **Signature** | Ed25519 | CRYSTALS-Dilithium | Ed25519 + Dilithium |
| **Encapsulation de clé** | X25519 | CRYSTALS-Kyber | X25519 + Kyber |
| **Hachage** | Blake3 | Blake3 | Blake3 |

#### 2.3 Stratégie de rotation des clés
- **Rotation d'urgence** : Une rotation des clés à l'échelle du réseau peut être déclenchée immédiatement en cas de menace quantique détectée
- **Rotation régulière** : Rotation des clés à long terme tous les 90 jours par défaut
- **Sécurité avant-coureuse** : Les clés de session utilisent PQC KEM pour garantir la sécurité avant-coureuse

### Feuille de route de migration
```
Phase 1 (actuelle) : Cryptographie classique comme principale
    ↓
Phase 2 : Mode mixte (support simultané classique et PQC)
    ↓
Phase 3 : PQC comme principal, classique comme secours
    ↓
Phase 4 : PQC complet (suppression du support des algorithmes classiques)
```

---

## III. Guide d'implémentation de sécurité client (couche sous-réseau AI Company)

### Positionnement couche protocole Skynet
Le protocole Skynet **définit les interfaces**, mais laisse l'implémentation spécifique au client et à la couche sous-réseau.

### Spécifications couche sous-réseau AI Company

#### 3.1 Spécifications de gestion des clés

##### Niveaux de stockage des clés
| Type de clé | Emplacement de stockage | Protection par chiffrement |
|-------------|-------------------------|---------------------------|
| **Clé privée d'identité** | Porte-clés système / HSM / TEE | Protection au niveau matériel |
| **Clé privée de dispositif** | Stockage local chiffré | Clé dérivée du mot de passe utilisateur |
| **Clé de session** | Mémoire (isolation de processus) | Destruction automatique |

##### Exigences de dérivation de clé
- **Algorithme** : Utilisation obligatoire d'Argon2id
- **Paramètres** :
  - Mémoire : Minimum 64 Mo
  - Nombre d'itérations : Minimum 3
  - Parallélisme : 4
  - Sel : Généré par CSPRNG de 16 octets

##### Sauvegarde et récupération des clés
- **Partage de secret Shamir** : Diviser la clé de récupération en 5 parts, 3 suffisent pour la récupération
- **Stockage hors ligne** : Encourager les utilisateurs à imprimer ou stocker la clé de récupération sur un dispositif hors ligne
- **Récupération sociale** : Prise en charge optionnelle de l'assistance à la récupération par des contacts de confiance

#### 3.2 Spécifications de gestion des clés API (spécifiques à AI Company)

##### Principes de sécurité des clés API
| Principe | Description |
|----------|-------------|
| **Privilège minimum** | Chaque clé API ne reçoit que les privilèges minimum nécessaires pour accomplir sa tâche |
| **Validité temporaire** : Prioriser l'utilisation de jetons à court terme plutôt que de clés API permanentes |
| **Utilisation isolée** : Utiliser des clés API différentes pour différents Agents/services |
| **Révocable** : Possibilité de révoquer à tout moment les clés API compromises ou non nécessaires |

##### Stockage des clés API
- **Emplacement de stockage** : Porte-clés système / Service de gestion de clés sécurisé (KMS)
- **Protection par chiffrement** : Chiffrement au repos, chiffrement des clés API avec une clé principale
- **Protection mémoire** : Effacement immédiat de la mémoire après utilisation pour éviter l'échange sur disque

##### Gestion du cycle de vie des clés API
```
Création → Distribution → Utilisation → Rotation → Révocation → Destruction
```

**Création** :
- Génération d'une clé API de force suffisante avec CSPRNG (au moins 256 bits d'entropie)
- Enregistrement de la date de création, du créateur, de l'usage, de la portée des privilèges

**Distribution** :
- Transmission via canal sécurisé (chiffrement de bout en bout)
- Éviter d'exposer la clé API complète dans les journaux et les messages d'erreur
- N'afficher que les premiers/derniers caractères de la clé pour identification

**Utilisation** :
- Injection via variables d'environnement ou fichier de configuration sécurisé plutôt que codage en dur
- Transmission via en-tête de requête ou champ d'autorisation (comme `Authorization: Bearer <clé>`)
- Éviter la transmission dans les paramètres de requête URL

**Rotation** :
- Rotation régulière (90 jours par défaut)
- Prise en charge d'une période de transition avec coexistence des anciennes et nouvelles clés (au moins 7 jours)
- Rotation immédiate en cas d'événement de sécurité déclenché

**Révocation** :
- Fournir une interface d'auto-révocation
- Invalidation immédiate après révocation
- Enregistrement de la raison et de la date de révocation

**Destruction** :
- Suppression de tous les emplacements de stockage
- Écrasement des résidus en mémoire et sur disque
- Vérification de l'achèvement de la destruction

##### Réponse d'urgence à la compromission de clé API
1. **Révocation immédiate** : Révoquer dès la détection de la compromission
2. **Journal d'audit** : Vérifier tous les enregistrements d'utilisation de cette clé
3. **Évaluation de l'impact** : Évaluer les pertes potentielles causées par la compromission
4. **Notification aux utilisateurs** : Notifier les utilisateurs/équipes affectés
5. **Analyse de la cause racine** : Identifier la cause de la compromission pour prévenir la récurrence
6. **Renforcement de la sécurité** : Améliorer les méthodes de stockage et d'utilisation

##### Priorité aux jetons temporaires
Pour les scénarios Agent, prioriser l'utilisation de jetons à court terme plutôt que de clés API permanentes :
- **OAuth 2.0** : Utiliser Client Credentials ou Authorization Code Flow
- **JWT** : Jeton JWT à court terme (validité < 1 heure)
- **Actualisation dynamique** : Utiliser Refresh Token pour obtenir un nouveau Access Token

#### 3.3 Spécifications de génération de nombres aléatoires

##### Exigences de source d'entropie
- **Source d'entropie principale** : CSPRNG du système d'exploitation (Windows : `CryptGenRandom`, Linux : `/dev/urandom`)
- **Sources d'entropie supplémentaires** :
  - Données de capteurs de dispositif (accéléromètre, gyroscope)
  - Horodatages d'interaction utilisateur
  - Mesures de latence réseau
- **Mélange d'entropie** : Utiliser les algorithmes Fortuna ou Yarrow pour mélanger l'entropie de multiples sources

##### Validation CSPRNG
- Implémentation de la norme NIST SP 800-90A
- Tests de régularité périodiques (Dieharder, TestU01)
- Interdiction d'utiliser tout générateur de nombres aléatoires non sécurisé du point de vue cryptographique

#### 3.4 Implémentation de référence (SDK sous-réseau AI Company)
Fournir un SDK officiel contenant :
- **Encapsulation de bibliothèque de chiffrement** : Interface unifiée pour les primitives cryptographiques
- **Module de gestion des clés** : Stockage sécurisé des clés prêt à l'emploi
- **Module de gestion des clés API** : Gestion sécurisée du cycle de vie des clés API

---

## IV. Audit de sécurité et journaux (responsabilité d'AI Company)

### Positionnement couche protocole Skynet
Les journaux d'audit **ne sont pas intégrés au protocole Skynet**, mais sont entièrement à la charge des créateurs de sous-réseaux selon leurs besoins.

### Raison
Les besoins d'audit varient considérablement selon les types de sous-réseaux :

| Type de sous-réseau | Besoins d'audit | Exemple |
|---------------------|-----------------|---------|
| **Sous-réseau organisationnel (AI Company)** | Exigences élevées | Journaux d'audit complets, inviolables, conservation pluriannuelle, certification de conformité |
| **Sous-réseau communautaire** | Exigences moyennes | Journaux d'opérations de base, conservation optionnelle |
| **Sous-réseau privé** | Exigences faibles | Peut ne nécessiter aucun journal d'audit |

### Implémentation par le sous-réseau AI Company

En tant que sous-réseau d'entreprise, AI Company est responsable de l'implémentation d'une fonctionnalité complète de journaux d'audit :

#### 4.1 Types d'événements d'audit
- Événements d'authentification d'identité
- Opérations sur les clés/clés API
- Modifications de privilèges
- Accès à des ressources sensibles

#### 4.2 Exigences de sécurité des journaux
- **Stockage local client** : Les journaux d'audit sont collectés et stockés par le client
- **Protection par chiffrement** : Stockage chiffré des journaux d'audit sensibles
- **Protection de l'intégrité** : Utilisation de signatures numériques pour empêcher la falsification
- **Configuration au niveau sous-réseau** : Les créateurs de sous-réseaux peuvent définir des stratégies d'audit

---

## V. Protection contre les attaques Sybil (responsabilité d'AI Company)

### Positionnement couche protocole Skynet
Le protocole Skynet **n'impose pas** de protection contre les attaques Sybil, préservant ainsi la flexibilité du protocole.

### Implémentation par la couche sous-réseau AI Company

#### 5.1 Système de protection multicouche

| Niveau de protection | Mécanisme | Description |
|---------------------|-----------|-------------|
| **Première couche** | Vérification d'identité | E-mail, numéro de téléphone, WebAuthn, etc. |
| **Deuxième couche** | Preuve de travail | Hashcash ou mécanismes similaires |
| **Troisième couche** | Vérification de graphe social | Approbation par des contacts de confiance |
| **Quatrième couche** | Garantie économique | Verrouillage de tokens comme dépôt de garantie |

#### 5.2 Options de vérification d'identité
- **WebAuthn/FIDO2** : Méthode d'authentification forte recommandée
- **Vérification par e-mail** : Envoi d'un lien de vérification
- **Vérification par numéro de téléphone** : Code de vérification par SMS
- **OAuth** : Prise en charge des fournisseurs d'identité tiers (Google, GitHub, etc.)

#### 5.3 Système de réputation
- **Réputation initiale** : Réputation basse pour les nouveaux utilisateurs, limitation de la fréquence des opérations
- **Accumulation de réputation** : Augmentation progressive de la réputation avec une utilisation normale
- **Sanction de réputation** : Réduction de la réputation pour les comportements malveillants
- **Réputation inter-sous-réseaux** : La réputation peut être partagée entre différents sous-réseaux

---

## VI. Renforcement de la confidentialité avant-coureuse (guide d'implémentation couche sous-réseau)

### Positionnement couche protocole Skynet
Le protocole Skynet **utilise** Double Ratchet et MLS pour fournir la confidentialité avant-coureuse, mais les détails d'implémentation spécifiques sont à la charge du client.

### Spécifications couche sous-réseau AI Company

#### 6.1 Spécifications de destruction de clés

##### Destruction de clés en mémoire
- **Écrasement à zéro** : Utiliser memset_s ou une fonction équivalente pour écraser la mémoire de la clé
- **Écrasements multiples** : Au moins 3 écrasements avec des motifs différents
- **Verrouillage mémoire** : Utiliser mlock/VirtualLock pour empêcher l'échange de la clé sur disque

##### Destruction de clés persistées
- **Suppression sécurisée** : Utiliser shred ou un outil équivalent
- **Traitement SSD** : Traitement spécifique pour SSD (TRIM, effacement sécurisé)
- **Vérification de destruction** : Confirmer que le matériel de clé ne peut plus être récupéré

#### 6.2 Garantie de confidentialité avant-coureuse
- **Clés de session à courte durée** : Rotation des clés à chaque message (Double Ratchet)
- **Pas de cache de clés** : Ne pas mettre en cache de clés pouvant être utilisées pour déchiffrer d'anciens messages
- **Confidentialité avant-coureuse parfaite (PFS)** : Garantie explicite de la propriété PFS

---

## VII. Limites de sécurité et défense en profondeur (couche sous-réseau AI Company)

### 7.1 Définition des limites de sécurité

| Limite | Mesures de protection |
|--------|-----------------------|
| **Limite client** | Isolation sandbox, isolation de processus, protection mémoire |
| **Limite réseau** | TLS 1.3, verrouillage de certificat, transparence des certificats |
| **Limite nœud de service** | Privilège minimum, isolation réseau, détection d'intrusion |
| **Limite données** | Chiffrement de bout en bout, chiffrement au repos, contrôle d'accès |

### 7.2 Stratégie de défense en profondeur

```
Première couche : Sécurité client
    ↓
Deuxième couche : Sécurité de transmission (TLS 1.3 + Noise)
    ↓
Troisième couche : Chiffrement de bout en bout (X3DH + Double Ratchet / MLS)
    ↓
Quatrième couche : Vérification multi-nœuds
    ↓
Cinquième couche : Journaux d'audit et détection d'anomalies (optionnel)
```

---

## VIII. Conformité sécurité et certification (couche sous-réseau AI Company)

### 8.1 Normes de conformité (satisfaites à la demande)
- **GDPR** : Protection des données, contrôle des utilisateurs, portabilité
- **HIPAA** (si applicable) : Protection des données médicales
- **SOC 2** : Sécurité, disponibilité, confidentialité
- **ISO 27001** : Système de gestion de la sécurité de l'information

### 8.2 Certification sécurité (optionnelle)
- **Audit tiers** : Audits de sécurité indépendants réguliers
- **Récompense pour vulnérabilités** : Établissement d'un programme de récompense pour vulnérabilités
- **Transparence publique** : Publication des rapports d'audit de sécurité

---

## Résumé

### Revue des responsabilités par couche

| Couche | Responsabilités |
|--------|-----------------|
| **Couche protocole Skynet** | Fournir une base légère et flexible : primitives cryptographiques, communication réseau, abstraction de sous-réseau |
| **Couche sous-réseau AI Company** | Implémentation de sécurité d'entreprise : gestion des clés API, journaux d'audit, certification de conformité, etc. |

### Division claire des responsabilités

| Domaine de sécurité | Protocole Skynet | AI Company |
|---------------------|------------------|------------|
| **Protection des métadonnées** | Ne fournit pas | Responsable de l'implémentation |
| **Résistance à la cryptanalyse quantique** | Réserve des points d'extension | Responsable de l'implémentation |
| **Gestion des clés API** | Ne s'applique pas | Responsable |
| **Journaux d'audit** | Non intégré | Responsable |
| **Protection Sybil** | N'impose pas | Responsable |

### Complément du modèle de sécurité AI Company

En tant qu'implémentation de sous-réseau d'entreprise sur Skynet, le modèle de sécurité AI Company complète la couche protocole de la manière suivante :

1. **Protection des métadonnées** : AI Company est responsable de l'implémentation, fournissant des mesures de base et une planification à long terme
2. **Migration post-quantique** : AI Company est responsable de l'implémentation, mode certificat mixte, feuille de route de migration progressive
3. **Sécurité client** : Spécifications de gestion des clés, spécifications de gestion des clés API, spécifications de génération de nombres aléatoires, SDK officiel
4. **Journaux d'audit** : AI Company est responsable de l'implémentation, fonctionnalité complète de journaux d'audit
5. **Protection Sybil** : AI Company est responsable de l'implémentation, protection multicouche, vérification d'identité, système de réputation
6. **Confidentialité avant-coureuse** : Spécifications de destruction de clés, garantie PFS
7. **Certification de conformité** : GDPR, SOC 2, ISO 27001

La combinaison des deux forme un système de sécurité d'entreprise complet, tout en préservant la légèreté et la flexibilité du protocole Skynet lui-même.
