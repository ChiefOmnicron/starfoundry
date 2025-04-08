<template>
    <n-config-provider :theme="dark" :theme-overrides="themeOverrides" class="font_overwrite">
        <n-loading-bar-provider>
            <n-global-style />

            <n-layout position="absolute">
                <n-layout-header class="header" bordered>
                    <div
                        class="nav-header-text"
                    >
                        <a href="/" style="color: white; text-decoration: none;">
                            {{ headerName }}
                        </a>
                    </div>

                    <div v-if="!is_logged_in() && !isAppraisal()">
                        <n-button
                            text
                            @click="redirect_login"
                        >
                            Login with Eve
                        </n-button>

                        <n-button
                            @click="about"
                            quaternary
                            style="margin-left: 10px"
                        >
                            About
                        </n-button>
                    </div>
                    <div
                        v-if="is_logged_in() && !isAppraisal()"
                        class="nav-header-character"
                    >
                        <span class="nav-header-character-text">{{ whoami.character_name }}</span>

                        <eve-icon :id="whoami.character_id" character />

                        <n-button
                            @click="about"
                            quaternary
                            style="margin-left: 10px"
                        >
                            About
                        </n-button>
                    </div>
                    <div v-if="isAppraisal()">
                        <n-button
                            @click="about"
                            quaternary
                            style="margin-left: 10px"
                        >
                            About
                        </n-button>
                    </div>
                </n-layout-header>

                <n-layout position="absolute" style="top: 64px;" has-sider>
                    <n-layout-sider bordered v-if="is_logged_in()" :native-scrollbar="false">
                        <n-menu
                            :value="current_route"
                            :options="options"
                            :expanded-keys="['projects']"
                            :default-expanded-keys="['projects']"
                        />
                    </n-layout-sider>

                    <n-layout content-style="padding-left: 24px; padding-right: 24px;" :native-scrollbar="false">
                        <n-message-provider>
                            <loading-bar>
                                <n-result
                                    v-if="!is_logged_in() && !no_login_required()"
                                    title="You are not logged in"
                                    description="This tool requires you to login into your eve account"
                                    style="margin-top: 100px"
                                >
                                    <template #footer>
                                        <n-button @click="redirect_login">Login with eve</n-button>
                                    </template>
                                </n-result>

                                <router-view
                                    v-if="is_logged_in() || no_login_required()"
                                    style="margin-bottom: 50px"
                                    :key="$route.fullPath"
                                />
                            </loading-bar>
                        </n-message-provider>

                        <n-layout-footer
                            style="background-color: rgb(16, 16, 20); border-top: 1px solid rgba(255, 255, 255, 0.09);"
                            position="absolute"
                        >
                            <div
                                style="margin-top: 5px; margin-left: 10px; color: rgba(255, 255, 255, 0.2); font-size: 10px"
                            >
                                All
                                <n-button
                                    style="font-size: 10px"
                                    text
                                    type="info"
                                >
                                    <router-link :to="{ name: 'legal' }"
                                        style="color: inherit;
                                        text-decoration: none"
                                    >
                                        Eve related materials
                                    </router-link>
                                </n-button>

                                are property of
                                <n-button
                                    :href="'https://www.ccpgames.com/'"
                                    style="font-size: 10px"
                                    tag="a"
                                    target="_blank"
                                    text
                                    type="info"
                                    >
                                    CCP Games.
                                </n-button>
                                See

                                <n-button
                                    style="font-size: 10px"
                                    text
                                    type="info"
                                >
                                    <router-link :to="{ name: 'legal' }"
                                        style="color: inherit;
                                        text-decoration: none"
                                    >
                                        legal notice.
                                    </router-link>
                                </n-button>
                            </div>
                        </n-layout-footer>
                    </n-layout>
                </n-layout>
            </n-layout>
        </n-loading-bar-provider>
    </n-config-provider>
</template>

<script lang="ts">
import {
    darkTheme, type GlobalThemeOverrides, NButton, NConfigProvider, NGlobalStyle,
    NLayout, NLayoutFooter, NLayoutHeader, NLayoutSider, NMenu, NMessageProvider, NResult,
    NLoadingBarProvider
} from 'naive-ui';
import { Component, Vue, toNative } from 'vue-facing-decorator'
import { h } from 'vue';
import { RouterLink } from 'vue-router';
import { events } from '@/main';
import { ROUTE_CHANGE, PROJECT_ROUTE } from '@/event_bus';

import { type Uuid } from './sdk/utils';
import { Service, type ICharacter } from '@/characters/service';


import * as industry_routes from '@/industry/router';
import * as project_routes from '@/project/router';
import { ROUTE_CHARACTERS } from '@/characters/router';
import { ROUTE_JOB_DETECTION_LIST } from './job_detection/router';
import { ROUTE_NOTIFICATIONS } from './notification/router';
import { ROUTE_PROJECT_GROUPS } from './project_group/router';
import { ROUTE_STOCK_BLUEPRINTS } from './stock/router';
import { ROUTE_STRUCTURE_DYNAMIC_GROUPS } from '@/structure_dynamic_group/router';
import { ROUTE_STRUCTURE_GROUPS } from '@/structure_group/router';
import { ROUTE_STRUCTURES } from '@/structure/router';

import EveIcon from '@/components/EveIcon.vue';
import LoadingBar from './components/LoadingBar.vue';
import { ROUTE_APPRAISAL, ROUTE_APPRAISAL_EMPTY } from './appraisal/router';
import { ROUTE_ABOUT } from './router';

@Component({
    components: {
        NButton,
        NConfigProvider,
        NGlobalStyle,
        NLayout,
        NLayoutFooter,
        NLayoutHeader,
        NLayoutSider,
        NLoadingBarProvider,
        NMenu,
        NMessageProvider,
        NResult,

        RouterLink,

        EveIcon,
        LoadingBar,
    }
})
class App extends Vue {
    public dark = darkTheme;

    public themeOverrides: GlobalThemeOverrides = {
        Alert: {
            borderRadius: '0'
        },
        Button: {
            borderRadiusMedium: '0',
        },
        Card: {
            borderRadius: '0',
        },
        Menu: {
            borderRadius: '0',
        },
        Table: {
            borderRadius: '0',
        },
        Tag: {
            borderRadius: '0',
        },
        Input: {
            borderRadius: '0',
        },
        InputNumber: {
            borderRadius: '0',
        },
        Select: {
            borderRadius: '0',
            peers: {
                InternalSelection: {
                    borderRadius: '0',
                }
            }
        },
        DataTable: {
            borderRadius: '0'
        },
    };

    public headerName: string = 'StarFoundry';

    public whoami: ICharacter = <ICharacter>{};
    public current_route: string = '';

    public options: any = [];

    public async created() {
        if (window.location.host.indexOf('industry.dev') > -1) {
            this.themeOverrides.Layout = {
                headerColor: 'red'
            };
            this.headerName = 'StarFoundry DEV';
        } else if (window.location.host.indexOf('appraisal') > -1) {
            this.headerName = 'StarFoundry Appraisal';
        }

        // #v-ifdef VITE_APPRAISAL==false
        const default_options = [{
            label: () => h(
                RouterLink,
                {
                    to: {
                        name: project_routes.ROUTE_PROJECTS
                    }
                },
                { default: () => 'Projects' }
            ),
            key: project_routes.ROUTE_PROJECTS,
        }, {
            label: () => h(
                RouterLink,
                {
                    to: {
                        name: project_routes.ROUTE_PROJECT_READY_JOBS
                    }
                },
                { default: () => 'Industry Jobs' }
            ),
            key: project_routes.ROUTE_PROJECT_READY_JOBS,
        }, {
            label: () => h(
                RouterLink,
                {
                    to: {
                        name: ROUTE_PROJECT_GROUPS
                    }
                },
                { default: () => 'Project Groups' }
            ),
            key: ROUTE_PROJECT_GROUPS,
        }, {
            label: () => h(
                RouterLink,
                {
                    to: {
                        name: ROUTE_APPRAISAL_EMPTY,
                    }
                },
                { default: () => 'Appraisal' }
            ),
            key: ROUTE_APPRAISAL_EMPTY,
        }, {
            label: 'Industry',
            key: 'industry',
            type: 'group',
            children: [
                this.app_link(industry_routes.ROUTE_COST_ESTIMATE, 'Cost Estimate'),
                this.app_link(industry_routes.ROUTE_INDUSTRY_INDEX, 'Industry Index'),
                this.app_link(ROUTE_JOB_DETECTION_LIST, 'Job Detection'),
            ]
        }, {
            label: 'Stocks',
            key: 'Stock',
            type: 'group',
            children: [
                this.app_link(ROUTE_STOCK_BLUEPRINTS, 'Blueprints'),
            ]
        }, {
            label: 'Structure',
            key: 'structure',
            type: 'group',
            children: [
                this.app_link(ROUTE_STRUCTURES, 'Structures'),
                this.app_link(ROUTE_STRUCTURE_GROUPS, 'Groups'),
                this.app_link(ROUTE_STRUCTURE_DYNAMIC_GROUPS, 'Dynamic Groups'),
            ]
        }, {
            label: 'Settings',
            key: 'settings',
            type: 'group',
            children: [
                this.app_link(ROUTE_CHARACTERS, 'Characters'),
                this.app_link(ROUTE_NOTIFICATIONS, 'Notifications'),
            ]
        }];

        events.$on(ROUTE_CHANGE, (e: string) => {
            this.current_route = e;

            if (this.options && this.options[0]) {
                this.options[0].children = [];
            }
        });

        // Projects active
        events.$on(PROJECT_ROUTE, (e: string) => {
            this.options = default_options;

            if (e) {
                let projectId = <Uuid>this.$route.params.projectId;
                this.options[0].children = [
                    this.project_link(project_routes.ROUTE_PROJECT_OVERVIEW, 'Overview', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_JOB, 'Jobs', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_MARKET, 'Market', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_MISC, 'Miscellaneous', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_EXCESS, 'Excess', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_STOCK, 'Stocks', { projectId }),
                    this.project_link(project_routes.ROUTE_PROJECT_SETTINGS, 'Settings', { projectId }),
                ];
                this.current_route = e;
            } else {
                this.current_route = 'projects';
                this.options[0].children = [];
            }
        });

        await this.whoami_req()
            .then(_ => {
                this.options = default_options;
            })
            .catch(e => {
                this.options = []
            });
        // #v-endif
    }

    public redirect_login() {
        // TODO: Move to character seervice
        window.location.href = `/api/v1/auth/login`;
    }

    // FIXME:
    public is_logged_in(): boolean {
        // TODO: not very secure
        return !!this.whoami.character_id;
    }

    // FIXME:
    public no_login_required(): boolean {
        if (this.isAppraisal()) {
            return true;
        }

        return this.$route.name === project_routes.ROUTE_PROJECT_ASSIGNMENTS ||
            this.$route.name === project_routes.ROUTE_PROJECT_STATISTICS ||
            this.$route.name === ROUTE_APPRAISAL ||
            this.$route.name === ROUTE_APPRAISAL_EMPTY;
    }

    public async whoami_req() {
        if (this.isAppraisal()) {
            return;
        }

        await Service
            .whoami()
            .then(x => {
                this.whoami = x;
                (<any>window).whoami = x;
            });
    }

    private app_link(to: string, name: string) {
        return {
            label: () =>
                h(
                    RouterLink,
                    {
                        to: {
                            name: to,
                        }
                    },
                    { default: () => name }
                ),
            key: to,
        };
    }

    private project_link(to: string, name: string, params: any) {
        return {
            label: () =>
                h(
                    RouterLink,
                    {
                        to: {
                            name: to,
                            params
                        }
                    },
                    { default: () => name }
                ),
            key: to,
        };
    }

    public about() {
        this.$router.push({
            name: ROUTE_ABOUT,
        });
    }

    public isAppraisal(): boolean {
        return window.location.host.indexOf('appraisal') > -1;
    }
}

export default toNative(App);
</script>

<style scoped>
.font_overwrite {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}

.header {
    cursor: pointer;

    height: 64px;
    padding: 24px;

    display: flex;
    align-items: center;
    justify-content: space-between;
}

.nav-header-text {
    font-size: 28px;
}

.nav-header-character {
    display: flex;
    align-items: center;
}

.nav-header-character-text {
    margin-right: 10px;
    font-size: 16px;
}
</style>
