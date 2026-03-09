import { defineConfig } from "vitepress";

export default defineConfig({
    title: "AI Company",
    description: "AI Company Official Documentation",
    head: [["link", { rel: "icon", href: "/vite.svg" }]],
    ignoreDeadLinks: true,
    rewrites: {
        "en/:slug*": ":slug*"
    },
    locales: {
        root: {
            label: "English",
            lang: "en",
            link: "/",
            description: "AI Company Official Documentation",
            themeConfig: {
                nav: [
                    { text: "Home", link: "/overview/" },
                    { text: "Advanced", link: "/advanced/" },
                    { text: "Developer Docs", link: "/maintainer/" },
                ],
                sidebar: {
                    "/tutorials/": [
                        {
                            text: "Overview",
                            items: [
                                { text: "Overview", link: "/overview/" },
                                { text: "Super Individual", link: "/overview/super-individual" },
                                { text: "Solo Company", link: "/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Tutorials",
                            items: [
                                { text: "Tutorials", link: "/tutorials/" },
                                { text: "User Workflow", link: "/tutorials/user-workflow" },
                                { text: "Use Cases", link: "/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/maintainer/": [
                        {
                            text: "Maintainer Docs",
                            items: [
                                { text: "Maintainer Docs", link: "/maintainer/" },
                                { text: "Data Models & Storage", link: "/maintainer/data-models" },
                                { text: "Agent Core", link: "/maintainer/agent-core" },
                                { text: "Technology Choices", link: "/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "Architecture",
                            items: [
                                { text: "Architecture", link: "/maintainer/architecture/" },
                                { text: "Decentralization", link: "/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet Protocol",
                            items: [
                                { text: "Skynet Protocol", link: "/maintainer/skynet/" },
                                { text: "Skynet Design", link: "/maintainer/skynet/skynet" },
                                { text: "Subnets", link: "/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/": [
                        {
                            text: "Overview",
                            items: [
                                { text: "Overview", link: "/overview/" },
                                { text: "Super Individual", link: "/overview/super-individual" },
                                { text: "Solo Company", link: "/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Tutorials",
                            items: [
                                { text: "Tutorials", link: "/tutorials/" },
                                { text: "User Workflow", link: "/tutorials/user-workflow" },
                                { text: "Use Cases", link: "/tutorials/use-cases/" },
                            ],
                        },
                        {
                            text: "Advanced",
                            items: [
                                { text: "Advanced", link: "/advanced/" },
                                { text: "Core Concepts", link: "/concepts/" },
                                { text: "Organization", link: "/concepts/organization" },
                                { text: "Project", link: "/concepts/project" },
                                { text: "Team", link: "/concepts/agent-cluster" },
                                { text: "Employee", link: "/concepts/agent" },
                                { text: "Workers", link: "/concepts/workers" },
                                { text: "Workspaces", link: "/concepts/workspaces" },
                                { text: "Workflows", link: "/concepts/workflows" },
                                { text: "Schedules", link: "/concepts/schedules" },
                                { text: "Extensibility", link: "/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        en: {
            label: "English",
            lang: "en",
            description: "AI Company Official Documentation",
            themeConfig: {
                nav: [
                    { text: "Home", link: "/en/overview/" },
                    { text: "Advanced", link: "/en/advanced/" },
                    { text: "Developer Docs", link: "/en/maintainer/" },
                ],
                sidebar: {
                    "/en/tutorials/": [
                        {
                            text: "Overview",
                            items: [
                                { text: "Overview", link: "/en/overview/" },
                                { text: "Super Individual", link: "/en/overview/super-individual" },
                                { text: "Solo Company", link: "/en/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Tutorials",
                            items: [
                                { text: "Tutorials", link: "/en/tutorials/" },
                                { text: "User Workflow", link: "/en/tutorials/user-workflow" },
                                { text: "Use Cases", link: "/en/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/en/maintainer/": [
                        {
                            text: "Maintainer Docs",
                            items: [
                                { text: "Maintainer Docs", link: "/en/maintainer/" },
                                { text: "Data Models & Storage", link: "/en/maintainer/data-models" },
                                { text: "Agent Core", link: "/en/maintainer/agent-core" },
                                { text: "Technology Choices", link: "/en/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "Architecture",
                            items: [
                                { text: "Architecture", link: "/en/maintainer/architecture/" },
                                { text: "Decentralization", link: "/en/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet Protocol",
                            items: [
                                { text: "Skynet Protocol", link: "/en/maintainer/skynet/" },
                                { text: "Skynet Design", link: "/en/maintainer/skynet/skynet" },
                                { text: "Subnets", link: "/en/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/en/": [
                        {
                            text: "Overview",
                            items: [
                                { text: "Overview", link: "/en/overview/" },
                                { text: "Super Individual", link: "/en/overview/super-individual" },
                                { text: "Solo Company", link: "/en/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Tutorials",
                            items: [
                                { text: "Tutorials", link: "/en/tutorials/" },
                                { text: "User Workflow", link: "/en/tutorials/user-workflow" },
                                { text: "Use Cases", link: "/en/tutorials/use-cases/" },
                            ],
                        },
                        {
                            text: "Advanced",
                            items: [
                                { text: "Advanced", link: "/en/advanced/" },
                                { text: "Core Concepts", link: "/en/concepts/" },
                                { text: "Organization", link: "/en/concepts/organization" },
                                { text: "Project", link: "/en/concepts/project" },
                                { text: "Team", link: "/en/concepts/agent-cluster" },
                                { text: "Employee", link: "/en/concepts/agent" },
                                { text: "Workers", link: "/en/concepts/workers" },
                                { text: "Workspaces", link: "/en/concepts/workspaces" },
                                { text: "Workflows", link: "/en/concepts/workflows" },
                                { text: "Schedules", link: "/en/concepts/schedules" },
                                { text: "Extensibility", link: "/en/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        "zh-hans": {
            label: "简体中文",
            lang: "zh-Hans",
            description: "AI Company 文档",
            themeConfig: {
                nav: [
                    { text: "首页", link: "/zh-hans/overview/" },
                    { text: "进阶技巧", link: "/zh-hans/advanced/" },
                    { text: "开发者文档", link: "/zh-hans/maintainer/" },
                ],
                sidebar: {
                    "/zh-hans/tutorials/": [
                        {
                            text: "概述",
                            items: [
                                { text: "概述", link: "/zh-hans/overview/" },
                                { text: "超级个体", link: "/zh-hans/overview/super-individual" },
                                { text: "一人公司", link: "/zh-hans/overview/solo-company" },
                            ],
                        },
                        {
                            text: "教程",
                            items: [
                                { text: "教程", link: "/zh-hans/tutorials/" },
                                { text: "用户工作流", link: "/zh-hans/tutorials/user-workflow" },
                                { text: "使用场景", link: "/zh-hans/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/zh-hans/maintainer/": [
                        {
                            text: "维护者文档",
                            items: [
                                { text: "维护者文档", link: "/zh-hans/maintainer/" },
                                { text: "数据模型与存储", link: "/zh-hans/maintainer/data-models" },
                                { text: "智能体核心定义", link: "/zh-hans/maintainer/agent-core" },
                                { text: "技术选型", link: "/zh-hans/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "架构设计",
                            items: [
                                { text: "架构设计", link: "/zh-hans/maintainer/architecture/" },
                                { text: "去中心化设计", link: "/zh-hans/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet 协议",
                            items: [
                                { text: "Skynet 协议", link: "/zh-hans/maintainer/skynet/" },
                                { text: "Skynet 协议设计", link: "/zh-hans/maintainer/skynet/skynet" },
                                { text: "子网模型", link: "/zh-hans/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/zh-hans/": [
                        {
                            text: "概述",
                            items: [
                                { text: "概述", link: "/zh-hans/overview/" },
                                { text: "超级个体", link: "/zh-hans/overview/super-individual" },
                                { text: "一人公司", link: "/zh-hans/overview/solo-company" },
                            ],
                        },
                        {
                            text: "教程",
                            items: [
                                { text: "教程", link: "/zh-hans/tutorials/" },
                                { text: "用户工作流", link: "/zh-hans/tutorials/user-workflow" },
                                { text: "使用场景", link: "/zh-hans/tutorials/use-cases/" },
                            ],
                        },
                        {
                            text: "进阶主题",
                            items: [
                                { text: "进阶主题", link: "/zh-hans/advanced/" },
                                { text: "核心概念", link: "/zh-hans/concepts/" },
                                { text: "组织", link: "/zh-hans/concepts/organization" },
                                { text: "项目", link: "/zh-hans/concepts/project" },
                                { text: "团队", link: "/zh-hans/concepts/agent-cluster" },
                                { text: "员工", link: "/zh-hans/concepts/agent" },
                                { text: "工作节点", link: "/zh-hans/concepts/workers" },
                                { text: "工作区", link: "/zh-hans/concepts/workspaces" },
                                { text: "工作流", link: "/zh-hans/concepts/workflows" },
                                { text: "日程/定时任务", link: "/zh-hans/concepts/schedules" },
                                { text: "扩展性与生态", link: "/zh-hans/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        "zh-hant": {
            label: "繁體中文",
            lang: "zh-Hant",
            description: "AI Company 官方文件",
            themeConfig: {
                nav: [
                    { text: "首頁", link: "/zh-hant/overview/" },
                    { text: "入門教程", link: "/zh-hant/tutorials/" },
                    { text: "進階技巧", link: "/zh-hant/advanced/" },
                    { text: "開發者文件", link: "/zh-hant/maintainer/" },
                ],
                sidebar: {
                    "/zh-hant/tutorials/": [
                        {
                            text: "教程",
                            items: [
                                { text: "教程", link: "/zh-hant/tutorials/" },
                                { text: "使用者工作流程", link: "/zh-hant/tutorials/user-workflow" },
                                { text: "使用場景", link: "/zh-hant/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/zh-hant/maintainer/": [
                        {
                            text: "維護者文件",
                            items: [
                                { text: "維護者文件", link: "/zh-hant/maintainer/" },
                                { text: "資料模型與儲存", link: "/zh-hant/maintainer/data-models" },
                                { text: "智慧體核心定義", link: "/zh-hant/maintainer/agent-core" },
                                { text: "技術選型", link: "/zh-hant/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "架構設計",
                            items: [
                                { text: "架構設計", link: "/zh-hant/maintainer/architecture/" },
                                { text: "去中心化設計", link: "/zh-hant/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet 協定",
                            items: [
                                { text: "Skynet 協定", link: "/zh-hant/maintainer/skynet/" },
                                { text: "Skynet 協定設計", link: "/zh-hant/maintainer/skynet/skynet" },
                                { text: "子網模型", link: "/zh-hant/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/zh-hant/": [
                        {
                            text: "概述",
                            items: [
                                { text: "概述", link: "/zh-hant/overview/" },
                                { text: "超級個體", link: "/zh-hant/overview/super-individual" },
                                { text: "一人公司", link: "/zh-hant/overview/solo-company" },
                            ],
                        },
                        {
                            text: "核心概念",
                            items: [
                                { text: "核心概念", link: "/zh-hant/concepts/" },
                                { text: "組織", link: "/zh-hant/concepts/organization" },
                                { text: "專案", link: "/zh-hant/concepts/project" },
                                { text: "團隊", link: "/zh-hant/concepts/agent-cluster" },
                                { text: "員工", link: "/zh-hant/concepts/agent" },
                                { text: "工作節點", link: "/zh-hant/concepts/workers" },
                                { text: "工作區", link: "/zh-hant/concepts/workspaces" },
                                { text: "工作流程", link: "/zh-hant/concepts/workflows" },
                                { text: "日程/定時任務", link: "/zh-hant/concepts/schedules" },
                            ],
                        },
                        {
                            text: "進階主題",
                            items: [
                                { text: "進階主題", link: "/zh-hant/advanced/" },
                                { text: "擴展性與生態", link: "/zh-hant/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        ja: {
            label: "日本語",
            lang: "ja",
            description: "AI Company 公式ドキュメント",
            themeConfig: {
                nav: [
                    { text: "ホーム", link: "/ja/overview/" },
                    { text: "入門チュートリアル", link: "/ja/tutorials/" },
                    { text: "上級テクニック", link: "/ja/advanced/" },
                    { text: "開発者ドキュメント", link: "/ja/maintainer/" },
                ],
                sidebar: {
                    "/ja/tutorials/": [
                        {
                            text: "チュートリアル",
                            items: [
                                { text: "チュートリアル", link: "/ja/tutorials/" },
                                { text: "ユーザーワークフロー", link: "/ja/tutorials/user-workflow" },
                                { text: "ユースケース", link: "/ja/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/ja/maintainer/": [
                        {
                            text: "メンテナードキュメント",
                            items: [
                                { text: "メンテナードキュメント", link: "/ja/maintainer/" },
                                { text: "データモデルとストレージ", link: "/ja/maintainer/data-models" },
                                { text: "エージェントコア", link: "/ja/maintainer/agent-core" },
                                { text: "技術選択", link: "/ja/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "アーキテクチャ",
                            items: [
                                { text: "アーキテクチャ", link: "/ja/maintainer/architecture/" },
                                { text: "分散化設計", link: "/ja/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet プロトコル",
                            items: [
                                { text: "Skynet プロトコル", link: "/ja/maintainer/skynet/" },
                                { text: "Skynet 設計", link: "/ja/maintainer/skynet/skynet" },
                                { text: "サブネット", link: "/ja/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/ja/": [
                        {
                            text: "概要",
                            items: [
                                { text: "概要", link: "/ja/overview/" },
                                { text: "スーパーインディビジュアル", link: "/ja/overview/super-individual" },
                                { text: "ソロカンパニー", link: "/ja/overview/solo-company" },
                            ],
                        },
                        {
                            text: "コアコンセプト",
                            items: [
                                { text: "コアコンセプト", link: "/ja/concepts/" },
                                { text: "組織", link: "/ja/concepts/organization" },
                                { text: "プロジェクト", link: "/ja/concepts/project" },
                                { text: "チーム", link: "/ja/concepts/agent-cluster" },
                                { text: "従業員", link: "/ja/concepts/agent" },
                                { text: "ワーカー", link: "/ja/concepts/workers" },
                                { text: "ワークスペース", link: "/ja/concepts/workspaces" },
                                { text: "ワークフロー", link: "/ja/concepts/workflows" },
                                { text: "スケジュール", link: "/ja/concepts/schedules" },
                            ],
                        },
                        {
                            text: "高度なトピック",
                            items: [
                                { text: "高度なトピック", link: "/ja/advanced/" },
                                { text: "拡張性とエコシステム", link: "/ja/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        ko: {
            label: "한국어",
            lang: "ko",
            description: "AI Company 공식 문서",
            themeConfig: {
                nav: [
                    { text: "홈", link: "/ko/overview/" },
                    { text: "입문 튜토리얼", link: "/ko/tutorials/" },
                    { text: "고급 기술", link: "/ko/advanced/" },
                    { text: "개발자 문서", link: "/ko/maintainer/" },
                ],
                sidebar: {
                    "/ko/tutorials/": [
                        {
                            text: "튜토리얼",
                            items: [
                                { text: "튜토리얼", link: "/ko/tutorials/" },
                                { text: "사용자 워크플로우", link: "/ko/tutorials/user-workflow" },
                                { text: "사용 사례", link: "/ko/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/ko/maintainer/": [
                        {
                            text: "메인테이너 문서",
                            items: [
                                { text: "메인테이너 문서", link: "/ko/maintainer/" },
                                { text: "데이터 모델과 스토리지", link: "/ko/maintainer/data-models" },
                                { text: "에이전트 코어", link: "/ko/maintainer/agent-core" },
                                { text: "기술 선택", link: "/ko/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "아키텍처",
                            items: [
                                { text: "아키텍처", link: "/ko/maintainer/architecture/" },
                                { text: "탈중앙화 설계", link: "/ko/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet 프로토콜",
                            items: [
                                { text: "Skynet 프로토콜", link: "/ko/maintainer/skynet/" },
                                { text: "Skynet 설계", link: "/ko/maintainer/skynet/skynet" },
                                { text: "서브넷", link: "/ko/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/ko/": [
                        {
                            text: "개요",
                            items: [
                                { text: "개요", link: "/ko/overview/" },
                                { text: "슈퍼 개인", link: "/ko/overview/super-individual" },
                                { text: "1인 기업", link: "/ko/overview/solo-company" },
                            ],
                        },
                        {
                            text: "핵심 개념",
                            items: [
                                { text: "핵심 개념", link: "/ko/concepts/" },
                                { text: "조직", link: "/ko/concepts/organization" },
                                { text: "프로젝트", link: "/ko/concepts/project" },
                                { text: "팀", link: "/ko/concepts/agent-cluster" },
                                { text: "직원", link: "/ko/concepts/agent" },
                                { text: "워커", link: "/ko/concepts/workers" },
                                { text: "워크스페이스", link: "/ko/concepts/workspaces" },
                                { text: "워크플로우", link: "/ko/concepts/workflows" },
                                { text: "스케줄", link: "/ko/concepts/schedules" },
                            ],
                        },
                        {
                            text: "고급 주제",
                            items: [
                                { text: "고급 주제", link: "/ko/advanced/" },
                                { text: "확장성과 생태계", link: "/ko/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        de: {
            label: "Deutsch",
            lang: "de",
            description: "AI Company Offizielle Dokumentation",
            themeConfig: {
                nav: [
                    { text: "Startseite", link: "/de/overview/" },
                    { text: "Erste Schritte", link: "/de/tutorials/" },
                    { text: "Fortgeschrittene", link: "/de/advanced/" },
                    { text: "Entwicklerdokumentation", link: "/de/maintainer/" },
                ],
                sidebar: {
                    "/de/tutorials/": [
                        {
                            text: "Tutorials",
                            items: [
                                { text: "Tutorials", link: "/de/tutorials/" },
                                { text: "Benutzer-Workflow", link: "/de/tutorials/user-workflow" },
                                { text: "Anwendungsfälle", link: "/de/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/de/maintainer/": [
                        {
                            text: "Maintainer-Dokumentation",
                            items: [
                                { text: "Maintainer-Dokumentation", link: "/de/maintainer/" },
                                { text: "Datenmodelle & Speicherung", link: "/de/maintainer/data-models" },
                                { text: "Agent-Kern", link: "/de/maintainer/agent-core" },
                                { text: "Technologieauswahl", link: "/de/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "Architektur",
                            items: [
                                { text: "Architektur", link: "/de/maintainer/architecture/" },
                                { text: "Dezentralisierung", link: "/de/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Skynet-Protokoll",
                            items: [
                                { text: "Skynet-Protokoll", link: "/de/maintainer/skynet/" },
                                { text: "Skynet-Design", link: "/de/maintainer/skynet/skynet" },
                                { text: "Subnetze", link: "/de/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/de/": [
                        {
                            text: "Übersicht",
                            items: [
                                { text: "Übersicht", link: "/de/overview/" },
                                { text: "Super Individual", link: "/de/overview/super-individual" },
                                { text: "Solo Company", link: "/de/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Kernkonzepte",
                            items: [
                                { text: "Kernkonzepte", link: "/de/concepts/" },
                                { text: "Organisation", link: "/de/concepts/organization" },
                                { text: "Projekt", link: "/de/concepts/project" },
                                { text: "Team", link: "/de/concepts/agent-cluster" },
                                { text: "Mitarbeiter", link: "/de/concepts/agent" },
                                { text: "Worker", link: "/de/concepts/workers" },
                                { text: "Arbeitsbereiche", link: "/de/concepts/workspaces" },
                                { text: "Arbeitsabläufe", link: "/de/concepts/workflows" },
                                { text: "Zeitpläne", link: "/de/concepts/schedules" },
                            ],
                        },
                        {
                            text: "Erweitert",
                            items: [
                                { text: "Erweitert", link: "/de/advanced/" },
                                { text: "Erweiterbarkeit", link: "/de/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        fr: {
            label: "Français",
            lang: "fr",
            description: "Documentation officielle AI Company",
            themeConfig: {
                nav: [
                    { text: "Accueil", link: "/fr/overview/" },
                    { text: "Démarrage", link: "/fr/tutorials/" },
                    { text: "Avancé", link: "/fr/advanced/" },
                    { text: "Documentation développeur", link: "/fr/maintainer/" },
                ],
                sidebar: {
                    "/fr/tutorials/": [
                        {
                            text: "Tutoriels",
                            items: [
                                { text: "Tutoriels", link: "/fr/tutorials/" },
                                { text: "Flux utilisateur", link: "/fr/tutorials/user-workflow" },
                                { text: "Cas d'utilisation", link: "/fr/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/fr/maintainer/": [
                        {
                            text: "Documentation des mainteneurs",
                            items: [
                                { text: "Documentation des mainteneurs", link: "/fr/maintainer/" },
                                { text: "Modèles de données & Stockage", link: "/fr/maintainer/data-models" },
                                { text: "Core Agent", link: "/fr/maintainer/agent-core" },
                                { text: "Choix technologiques", link: "/fr/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "Architecture",
                            items: [
                                { text: "Architecture", link: "/fr/maintainer/architecture/" },
                                { text: "Décentralisation", link: "/fr/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Protocole Skynet",
                            items: [
                                { text: "Protocole Skynet", link: "/fr/maintainer/skynet/" },
                                { text: "Conception Skynet", link: "/fr/maintainer/skynet/skynet" },
                                { text: "Sous-réseaux", link: "/fr/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/fr/": [
                        {
                            text: "Aperçu",
                            items: [
                                { text: "Aperçu", link: "/fr/overview/" },
                                { text: "Super Individu", link: "/fr/overview/super-individual" },
                                { text: "Solo Company", link: "/fr/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Concepts clés",
                            items: [
                                { text: "Concepts clés", link: "/fr/concepts/" },
                                { text: "Organisation", link: "/fr/concepts/organization" },
                                { text: "Projet", link: "/fr/concepts/project" },
                                { text: "Équipe", link: "/fr/concepts/agent-cluster" },
                                { text: "Employé", link: "/fr/concepts/agent" },
                                { text: "Workers", link: "/fr/concepts/workers" },
                                { text: "Espaces de travail", link: "/fr/concepts/workspaces" },
                                { text: "Flux de travail", link: "/fr/concepts/workflows" },
                                { text: "Horaires", link: "/fr/concepts/schedules" },
                            ],
                        },
                        {
                            text: "Avancé",
                            items: [
                                { text: "Avancé", link: "/fr/advanced/" },
                                { text: "Extensibilité", link: "/fr/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
        ru: {
            label: "Русский",
            lang: "ru",
            description: "Официальная документация AI Company",
            themeConfig: {
                nav: [
                    { text: "Главная", link: "/ru/overview/" },
                    { text: "Начало работы", link: "/ru/tutorials/" },
                    { text: "Продвинутые", link: "/ru/advanced/" },
                    { text: "Документация разработчика", link: "/ru/maintainer/" },
                ],
                sidebar: {
                    "/ru/tutorials/": [
                        {
                            text: "Учебники",
                            items: [
                                { text: "Учебники", link: "/ru/tutorials/" },
                                { text: "Рабочий процесс пользователя", link: "/ru/tutorials/user-workflow" },
                                { text: "Примеры использования", link: "/ru/tutorials/use-cases/" },
                            ],
                        },
                    ],
                    "/ru/maintainer/": [
                        {
                            text: "Документация для сопровождения",
                            items: [
                                { text: "Документация для сопровождения", link: "/ru/maintainer/" },
                                { text: "Модели данных и хранилище", link: "/ru/maintainer/data-models" },
                                { text: "Ядро агента", link: "/ru/maintainer/agent-core" },
                                { text: "Выбор технологий", link: "/ru/maintainer/technology-choices" },
                            ],
                        },
                        {
                            text: "Архитектура",
                            items: [
                                { text: "Архитектура", link: "/ru/maintainer/architecture/" },
                                { text: "Децентрализация", link: "/ru/maintainer/architecture/decentralization" },
                            ],
                        },
                        {
                            text: "Протокол Skynet",
                            items: [
                                { text: "Протокол Skynet", link: "/ru/maintainer/skynet/" },
                                { text: "Дизайн Skynet", link: "/ru/maintainer/skynet/skynet" },
                                { text: "Подсети", link: "/ru/maintainer/skynet/subnets" },
                            ],
                        },
                    ],
                    "/ru/": [
                        {
                            text: "Обзор",
                            items: [
                                { text: "Обзор", link: "/ru/overview/" },
                                { text: "Супер-индивид", link: "/ru/overview/super-individual" },
                                { text: "Соло-компания", link: "/ru/overview/solo-company" },
                            ],
                        },
                        {
                            text: "Основные понятия",
                            items: [
                                { text: "Основные понятия", link: "/ru/concepts/" },
                                { text: "Организация", link: "/ru/concepts/organization" },
                                { text: "Проект", link: "/ru/concepts/project" },
                                { text: "Команда", link: "/ru/concepts/agent-cluster" },
                                { text: "Сотрудник", link: "/ru/concepts/agent" },
                                { text: "Воркеры", link: "/ru/concepts/workers" },
                                { text: "Рабочие пространства", link: "/ru/concepts/workspaces" },
                                { text: "Рабочие процессы", link: "/ru/concepts/workflows" },
                                { text: "Расписания", link: "/ru/concepts/schedules" },
                            ],
                        },
                        {
                            text: "Расширенные темы",
                            items: [
                                { text: "Расширенные темы", link: "/ru/advanced/" },
                                { text: "Расширяемость и экосистема", link: "/ru/advanced/extensibility-ecosystem" },
                            ],
                        },
                    ],
                },
                socialLinks: [{ icon: "github", link: "https://github.com/" }],
            },
        },
    },
});
