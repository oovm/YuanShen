# Scénario 3 : Projet de recherche avec collaboration multi-appareils

## Description du scénario

Un chercheur, utilisant un réseau de nœuds de travail composé de plusieurs appareils pour mener un projet de recherche complexe.

## Architecture organisationnelle

- Modèle **Entreprise individuelle**
- **Secrétariat** : Responsable de la coordination globale et du contrôle de l'avancement de la recherche
- **Équipe de recherche bibliographique** : Responsable de la collecte et de l'organisation de la littérature
  - Agent de recherche : Rechercher la littérature pertinente
  - Agent d'organisation : Organiser les notes de littérature, générer une synthèse
- **Équipe de conception expérimentale** : Responsable de la conception expérimentale et de l'analyse des données
  - Agent de conception : Concevoir le plan expérimental
  - Agent d'analyse : Traiter les données expérimentales, générer des graphiques
- **Équipe de rédaction de thèse** : Responsable de la rédaction et de la modification de la thèse
  - Agent de rédaction : Rédiger le brouillon de la thèse
  - Agent de peaufinage : Optimiser le langage, améliorer l'expression

## Configuration des nœuds de travail

- **Nœud de travail A (Serveur haute performance)** :
  - Espace de travail : Calcul expérimental
  - Agent en cours d'exécution : Agent d'analyse (traiter de grandes quantités de données)
- **Nœud de travail B (Ordinateur portable personnel)** :
  - Espace de travail : Recherche bibliographique
  - Agents en cours d'exécution : Agent de recherche, Agent d'organisation
- **Nœud de travail C (Appareil de secours)** :
  - Espace de travail : Rédaction de thèse
  - Agents en cours d'exécution : Agent de rédaction, Agent de peaufinage

## Flux de travail

1. **Recherche bibliographique** : Sur le nœud de travail B, rechercher et organiser la littérature pertinente
2. **Conception expérimentale** : Concevoir le plan expérimental, préparer les données expérimentales
3. **Analyse des données** : Sur le nœud de travail A, exécuter la tâche d'analyse des données
4. **Rédaction de thèse** : Sur le nœud de travail C, rédiger la thèse
5. **Contrôle de qualité** : Le secrétariat examine tous les résultats pour garantir la qualité
