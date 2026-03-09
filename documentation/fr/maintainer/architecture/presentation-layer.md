# Couche de Présentation (Côté Application)

La couche de présentation est l'interface avec laquelle l'utilisateur interagit directement, incluant divers programmes d'application et interfaces frontales. Cette couche est située au sommet de l'architecture à trois couches et fournit des services en appelant les modules centraux de la couche d'implémentation.

## Architecture Générale

La couche de présentation est composée de deux parties principales :
- **Applications frontales** - Situées dans le répertoire `frontends/`, fournissant l'interface utilisateur
- **Applications backends** - Situées dans le répertoire `backends/`, fournissant le support côté serveur

### Diagramme de Relation des Composants de la Couche de Présentation

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Frontale (frontends/)          │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (App principale) │  │  (Module Empire) │  │  (Module Planète) │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-waifu    │  │  client-h5    │  │  client-desktop│  │
│  │  (Module Personnage) │  │  (Client H5)   │  │  (Client Bureau)  │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │client-mobile │  │ client-shared │                        │
│  │(Mobile)      │  │ (Bibliothèque partagée) │              │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/WebSocket
┌─────────────────────────────────────────────────────────────┐
│                    Application Backend (backends/)             │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │  ai-company  │  │  ai-empire   │  │  ai-planet   │    │
│  │  (App principale) │  │  (Module Empire) │  │  (Module Planète) │
│  └──────────────┘  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐                                            │
│  │  ai-waifu    │                                            │
│  │  (Module Personnage) │                                    │
│  └──────────────┘                                            │
└─────────────────────────────────────────────────────────────┘
                            ↓ Appel
┌─────────────────────────────────────────────────────────────┐
│         Couche d'Implémentation (Modules Centraux augur-*)   │
└─────────────────────────────────────────────────────────────┘
```

## Détails des Applications Frontales

### 1. ai-company (Application Principale)

**Emplacement du répertoire** : `frontends/ai-company/`

**Pile technique** :
- Vue 3 + TypeScript
- Outil de construction Vite
- Bibliothèque de composants UI Element Plus
- Routage Vue Router
- Gestion d'état Pinia
- CSS atomique UnoCSS
- Internationalisation Fluent Vue

**Port** : Le port par défaut du serveur de développement est configuré via Vite

**Responsabilités fonctionnelles** :
- Affichage de la page d'accueil marketing
- Connexion/Inscription utilisateur
- Gestion de l'entreprise
- Gestion des employés
- Gestion des équipes
- Gestion des projets
- Fonctionnalité de forum
- Fonctionnalités d'administration

**Vues principales** :
- `Home.vue` - Page d'accueil
- `Login.vue` - Page de connexion
- `Register.vue` - Page d'inscription
- `Company.vue` - Liste des entreprises
- `CompanyDetail.vue` - Détails de l'entreprise
- `Employees.vue` - Liste des employés
- `EmployeeDetail.vue` - Détails de l'employé
- `Team.vue` - Liste des équipes
- `TeamDetail.vue` - Détails de l'équipe
- `Project.vue` - Liste des projets
- `ProjectDetail.vue` - Détails du projet
- `Forum.vue` - Forum
- `ForumDetail.vue` - Détails du forum
- `Dashboard.vue` - Tableau de bord
- `Download.vue` - Page de téléchargement
- `admin/AdminDashboard.vue` - Tableau de bord de l'administrateur
- `admin/Users.vue` - Gestion des utilisateurs

### 2. ai-empire (Module Empire)

**Emplacement du répertoire** : `frontends/ai-empire/`

**Pile technique** :
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Bibliothèque de composants UI Element Plus
- Routage Vue Router
- Gestion d'état Pinia
- CSS atomique UnoCSS
- Internationalisation Fluent Vue
- Éditeur de texte enrichi TipTap

**Responsabilités fonctionnelles** :
- Interface de gestion de l'empire
- Gestion des conseillers AI (Advisor)
- Gestion des consultants AI (Consultant)
- Gestion des légions (Legion)
- Gestion des grandes œuvres (Great Work)
- Authentification utilisateur
- Affichage du tableau de bord

**Vues principales** :
- `Home.vue` - Page d'accueil
- `Login.vue` - Page de connexion
- `Register.vue` - Page d'inscription
- `Empire.vue` - Liste des empires
- `EmpireDetail.vue` - Détails de l'empire
- `Advisors.vue` - Liste des conseillers
- `AdvisorDetail.vue` - Détails du conseiller
- `Consultants.vue` - Liste des consultants
- `ConsultantDetail.vue` - Détails du consultant
- `Legion.vue` - Liste des légions
- `LegionDetail.vue` - Détails de la légion
- `GreatWork.vue` - Liste des grandes œuvres
- `GreatWorkDetail.vue` - Détails de la grande œuvre

**Fonctionnalités spéciales** :
- Gestion des employés AI : Traitez les agents AI comme des employés d'entreprise
- Assistants AI personnalisés : Service client, finance, marketing, assistant administratif, droit, etc.
- Système de gestion des connaissances
- Fonctionnalité d'édition de texte enrichi

### 3. ai-planet (Module Planète)

**Emplacement du répertoire** : `frontends/ai-planet/`

**Pile technique** :
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Bibliothèque de composants UI Element Plus
- Routage Vue Router
- Gestion d'état Pinia
- CSS atomique UnoCSS
- Internationalisation Fluent Vue
- Éditeur de texte enrichi TipTap

**Responsabilités fonctionnelles** :
- Interface de fonctionnalités liées à la planète
- Authentification utilisateur

**Vues principales** :
- `Home.vue` - Page d'accueil
- `Auth.vue` - Page d'authentification

### 4. ai-waifu (Module Personnage)

**Emplacement du répertoire** : `frontends/ai-waifu/`

**Pile technique** :
- Vue 3 + TypeScript
- Vite + Vite-SSG
- Bibliothèque de composants UI Element Plus
- Routage Vue Router
- Gestion d'état Pinia
- CSS atomique UnoCSS
- Internationalisation Fluent Vue
- Éditeur de texte enrichi TipTap

**Responsabilités fonctionnelles** :
- Interface de chat de personnages AI
- Interaction avec les personnages

**Vues principales** :
- `Home.vue` - Page d'accueil
- `Chat.vue` - Interface de chat

### 5. client-h5 (Client H5)

**Emplacement du répertoire** : `frontends/client-h5/`

**Pile technique** :
- Vue 3 + TypeScript
- Outil de construction Vite
- Bibliothèque de composants UI Element Plus
- Routage Vue Router
- Gestion d'état Pinia
- CSS atomique UnoCSS

**Responsabilités fonctionnelles** :
- Application H5 mobile
- Vue de l'espace de travail
- Mode de collaboration Augur
- Mode de style WeChat Enterprise
- Gestion des applications
- Centre de messages
- Gestion des contacts
- Calendrier
- Réunions
- Courrier électronique
- Base de connaissances
- Gestion de la mémoire
- Gestion de projet

**Fonctionnalités spéciales** :
- Changement de mode de travail : Mode de collaboration Augur / Mode de style WeChat Enterprise
- Intégration multi-applications : Présence, CRM, contrat, finance, RH, performance, recrutement, Wiki, workflow, etc.
- Plusieurs modes de mise en page

### 6. client-desktop (Client Bureau)

**Emplacement du répertoire** : `frontends/client-desktop/`

**Pile technique** :
- Tauri (Rust + WebView)
- Application de bureau multiplateforme

**Responsabilités fonctionnelles** :
- Programme d'application de bureau
- Fournir une expérience de bureau native

### 7. client-mobile (Client Mobile)

**Emplacement du répertoire** : `frontends/client-mobile/`

**Pile technique** :
- Tauri (Rust + WebView)
- Application mobile

**Responsabilités fonctionnelles** :
- Programme d'application mobile

### 8. client-shared (Bibliothèque Partagée)

**Emplacement du répertoire** : `frontends/client-shared/`

**Pile technique** :
- Vue 3 + TypeScript

**Responsabilités fonctionnelles** :
- Bibliothèque de composants partagés
- Services partagés (appels API)
- Gestion d'état partagée (stores Pinia)
- Définitions de types partagées
- Fonctions utilitaires partagées
- Ressources d'internationalisation partagées
- Plugins partagés

**Contenu inclus** :
- `components/` - Composants Vue partagés
- `services/` - Services API et données de simulation
- `stores/` - Gestion d'état Pinia
- `types/` - Définitions de types TypeScript
- `utils/` - Fonctions utilitaires
- `locales/` - Ressources d'internationalisation
- `plugins/` - Plugins Vue

## Détails des Applications Backends

### 1. ai-company (Backend de l'Application Principale)

**Emplacement du répertoire** : `backends/ai-company/`

**Pile technique** :
- Rust + Runtime asynchrone Tokio
- Framework Web Axum
- Sérialisation Serde
- Journalisation Tracing
- Intégration de ressources statiques rust-embed

**Port** : 4002

**Responsabilités fonctionnelles** :
- Fournir un service HTTP
- Intégrer et fournir des ressources statiques frontales
- Routage et service de fichiers statiques
- Fournir un support côté serveur pour le frontend de l'application principale

**Fonctionnalités principales** :
- Service de fichiers statiques : Intègre les produits de construction frontend du répertoire `frontends/ai-company/dist`
- Prise en charge du routage d'application monopage : Tous les chemins non appariés renvoient index.html
- Détection automatique du type MIME des ressources statiques

### 2. ai-empire (Backend du Module Empire)

**Emplacement du répertoire** : `backends/ai-empire/`

**Pile technique** :
- Rust + Runtime asynchrone Tokio
- Framework Web Axum
- Sérialisation Serde
- Journalisation Tracing
- Intégration de ressources statiques rust-embed

**Responsabilités fonctionnelles** :
- Fournir un service HTTP
- Intégrer et fournir des ressources statiques frontend du module empire
- Fournir un support côté serveur pour le frontend du module empire

### 3. ai-planet (Backend du Module Planète)

**Emplacement du répertoire** : `backends/ai-planet/`

**Pile technique** :
- Rust + Runtime asynchrone Tokio
- Framework Web Axum
- Sérialisation Serde
- Journalisation Tracing
- Intégration de ressources statiques rust-embed

**Responsabilités fonctionnelles** :
- Fournir un service HTTP
- Intégrer et fournir des ressources statiques frontend du module planète
- Fournir un support côté serveur pour le frontend du module planète

### 4. ai-waifu (Backend du Module Personnage)

**Emplacement du répertoire** : `backends/ai-waifu/`

**Pile technique** :
- Rust + Runtime asynchrone Tokio
- Framework Web Axum
- Sérialisation Serde
- Journalisation Tracing
- Intégration de ressources statiques rust-embed

**Responsabilités fonctionnelles** :
- Fournir un service HTTP
- Intégrer et fournir des ressources statiques frontend du module personnage
- Fournir un support côté serveur pour le frontend du module personnage

## Relations entre les Applications

### Relations entre les Applications Frontales

```
client-shared (Bibliothèque partagée)
    ↑ Dépendance
    ├─→ ai-company (App principale)
    ├─→ ai-empire (Module Empire)
    ├─→ ai-planet (Module Planète)
    ├─→ ai-waifu (Module Personnage)
    └─→ client-h5 (Client H5)

client-h5 (Client H5)
    ↑ Référence/inspiration
    ├─→ client-desktop (Client Bureau)
    └─→ client-mobile (Client Mobile)
```

### Relation d'appariement Frontend-Backend

| Application Frontale | Application Backend | Description |
|---------------------|---------------------|-------------|
| `ai-company` | `ai-company` | Appariement frontend-backend de l'application principale |
| `ai-empire` | `ai-empire` | Appariement frontend-backend du module empire |
| `ai-planet` | `ai-planet` | Appariement frontend-backend du module planète |
| `ai-waifu` | `ai-waifu` | Appariement frontend-backend du module personnage |
| `client-h5` | (À déterminer) | Backend du client H5 |
| `client-desktop` | (À déterminer) | Backend du client bureau |
| `client-mobile` | (À déterminer) | Backend du client mobile |

### Relation avec la Couche d'Implémentation

Toutes les applications backends appellent finalement les modules centraux `augur-*` de la couche d'implémentation pour exécuter la logique métier :

```
Couche de Présentation (Applications backends)
    ↓ Appel
Couche d'Implémentation (Modules centraux augur-*)
    ↓ Utilise
Couche de Protocole (Protocole Skynet)
```

## Architecture de Déploiement

### Environnement de Développement

- Les applications frontales s'exécutent indépendamment via le serveur de développement Vite
- Les applications backends s'exécutent indépendamment via Cargo
- Frontend et backend communiquent via des appels API

### Environnement de Production

- Les produits de construction frontend sont intégrés dans l'application backend correspondante (via rust-embed)
- Chaque application backend s'exécute en tant que service indépendant, fournissant des fonctionnalités frontend et backend complètes
- Chaque service écoute sur un port différent

## Explication des Choix Techniques

### Choix Techniques Frontend

- **Vue 3** : Framework JavaScript progressif, fournissant une excellente expérience de développement et des performances
- **TypeScript** : Fournit une sécurité de type, améliore la maintenabilité du code
- **Vite** : Outil de construction frontend de nouvelle génération, fournissant une expérience de développement extrêmement rapide
- **Element Plus** : Bibliothèque de composants basée sur Vue 3, fournissant de riches composants UI
- **Pinia** : Bibliothèque de gestion d'état officiellement recommandée pour Vue 3
- **UnoCSS** : Moteur CSS atomique, fournissant un schéma de style flexible
- **Fluent Vue** : Solution d'internationalisation, prend en charge plusieurs langues

### Choix Techniques Backend

- **Rust** : Langage de programmation système haute performance et sécurisé en mémoire
- **Tokio** : Runtime asynchrone Rust, fournissant un traitement d'E/S haute performance
- **Axum** : Framework Web ergonomique et modulaire, fournissant une bonne expérience de développement
- **rust-embed** : Intègre les produits de construction frontend dans le fichier binaire Rust, simplifiant le déploiement

## Guide de Développement

### Développement Frontend

```bash
# Entrer dans le répertoire de l'application frontend
cd frontends/ai-company

# Installer les dépendances
npm install

# Démarrer le serveur de développement
npm run dev

# Construire la version de production
npm run build
```

### Développement Backend

```bash
# Entrer dans le répertoire de l'application backend
cd backends/ai-company

# Exécuter le serveur de développement
cargo run

# Construire la version de production
cargo build --release
```

### Flux de Développement Complet

1. D'abord construire l'application frontend
2. Puis exécuter l'application backend (qui intègre les produits de construction frontend)

```bash
# Construire le frontend
cd frontends/ai-company
npm run build

# Exécuter le backend
cd ../../backends/ai-company
cargo run
```
