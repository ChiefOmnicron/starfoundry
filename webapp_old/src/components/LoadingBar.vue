<template>
    <slot></slot>
</template>

<script lang="ts">
import { Component, Vue, toNative } from 'vue-facing-decorator';

import axios from 'axios';

import { useLoadingBar } from 'naive-ui';

@Component({
    components: {},
})
//export default class App extends Vue {
class LoadingBar extends Vue {
    public loadingBar = useLoadingBar();

    private counter = 0;

    public created() {
        axios.interceptors.request.use(
            (config) => {
                this.loadingBar.start();
                this.counter += 1;

                return config;
            },
            (error) => {
                this.loadingBar.error();
                return Promise.reject(error);
            },
        );

        axios.interceptors.response.use(
            (response) => {
                this.counter -= 1;

                if (this.counter <= 0) {
                    this.loadingBar.finish();
                    this.counter = 0;
                }

                return response;
            },
            (error) => {
                this.loadingBar.error();
                this.counter = 0;
                return Promise.reject(error);
            },
        );
    }
}

export default toNative(LoadingBar);
</script>
