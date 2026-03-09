# Employé (Employee) : Agent

Un employé (Employee) est un agent (Agent) composé d'un **ensemble de compétences**, définissant ce que l'agent peut faire, ce qu'il fait bien et comment il collabore avec d'autres agents.

## Définition du concept

Un **employé** est un conteneur et une combinaison de compétences, formant une capacité de travail complète en combinant différentes compétences. L'employé lui-même n'est pas une compétence, mais un ensemble de compétences.

## Éléments clés

### Informations de base

- **Nom de l'employé** : Nom identifiant de l'agent
- **Avatar de l'employé** : Identité visuelle
- **Poste** : Positionnement du rôle de l'agent
- **Description** : Présentation détaillée des capacités
- **Type de rôle** : Classification du rôle de l'agent

### Ensemble de compétences

L'employé forme une capacité de travail en combinant plusieurs compétences :

- **Compétences fondamentales** : Capacités de base (communication, raisonnement, apprentissage, mémoire, etc.)
- **Compétences professionnelles** : Capacités professionnelles dans des domaines spécifiques (programmation, analyse, conception, etc.)
- **Compétences outils** : Capacité à utiliser des outils spécifiques (outils de développement, outils de conception, outils bureautiques, etc.)
- **Compétences relationnelles** : Capacités d'interaction humaine et de collaboration d'équipe

Voir [skills.md](skills.md) pour en savoir plus sur le système de compétences détaillé.

### Configuration des capacités

L'employé peut configurer plusieurs capacités pour étendre ses fonctionnalités :

- **Capacité d'intégration API** : Capacité à appeler des API externes
- **Capacité de service MCP** : Capacité à utiliser les services MCP (Model Context Protocol)
- **Capacité d'opération de fichiers** : Capacité à lire et écrire des fichiers
- **Capacité d'accès réseau** : Capacité à accéder aux ressources réseau

Voir [capabilities.md](capabilities.md) pour en savoir plus sur le système de capacités détaillé.

### Style de travail

- **Vitesse de réponse** : Réponse rapide / Réflexion approfondie
- **Mode de décision** : Décision rapide / Consultation complète
- **Style de communication** : Concis et direct / Explication détaillée
- **Préférence de risque** : Aventureux / Prudent et conservateur

## Rôle de l'employé

### 1. Combinaison de compétences

- Combiner plusieurs compétences en une capacité de travail complète
- Les compétences collaborent entre elles, formant une force combinée
- S'adapter flexiblement à divers scénarios de tâches

### 2. Standardisation des capacités

- Définir clairement les limites des capacités de l'agent
- Faciliter le choix de l'agent approprié par l'utilisateur
- Peut évaluer et comparer différents agents

### 3. Évolution continue

- L'agent peut apprendre de nouvelles compétences
- Les compétences peuvent être continuellement améliorées et optimisées
- Peut ajouter de nouvelles extensions de capacités
- S'adapter à de nouveaux besoins et scénarios

## Exemple : Agent développeur full-stack

```
Développeur full-stack (Alex)
├── Informations de base
│   ├── Nom : Alex
│   ├── Poste : Ingénieur senior full-stack
│   └── Présentation : 5 ans d'expérience en développement, spécialisé dans le développement d'applications Web
├── Ensemble de compétences
│   ├── Compétences fondamentales
│   │   ├── Capacité de communication : Avancée
│   │   ├── Capacité de raisonnement : Expert
│   │   ├── Capacité d'apprentissage : Avancée
│   │   └── Capacité de mémoire : Avancée
│   ├── Compétences professionnelles
│   │   ├── Développement frontend (React/Vue/TypeScript) : Expert
│   │   ├── Développement backend (Node.js/Python) : Avancé
│   │   ├── Conception de bases de données : Expert
│   │   └── Conception API : Avancée
│   ├── Compétences outils
│   │   ├── Git : Expert
│   │   ├── VS Code : Expert
│   │   ├── Docker : Avancé
│   │   └── CI/CD : Avancé
│   └── Compétences relationnelles
│       ├── Collaboration d'équipe : Avancée
│       ├── Revue de code : Expert
│       └── Documentation technique : Avancée
├── Configuration des capacités
│   ├── Capacité d'intégration API : Activée
│   ├── Capacité de service MCP : Activée
│   ├── Capacité d'opération de fichiers : Activée
│   └── Capacité d'accès réseau : Activée
└── Style de travail
    ├── Vitesse de réponse : Réponse rapide
    ├── Mode de décision : Décision rapide
    ├── Style de communication : Explication détaillée
    └── Style de code : Focalisé sur la maintenabilité
```
