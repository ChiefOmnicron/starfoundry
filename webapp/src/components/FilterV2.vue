<template>
    <n-dropdown
        :options="filterOptions"
        :show="showOptions"
        @clickoutside="onClickOutside"
        @select="filterSelected"
        placement="bottom-start"
        trigger="manual"
    >
        <n-input
            @click="handleShowDropdown"
            @keydown="handleKeydown"
            data-cy="filter-text"
            placeholder="Filter"
            ref="filterInput"
            type="text"
            v-model:value="search"
        >
            <template #prefix>
                <n-icon>
                    <search />
                </n-icon>
            </template>

            <!--template #suffix>
                <n-icon @click="createFilterBookmark">
                    <bookmark-regular />
                </n-icon>
            </template-->
        </n-input>
    </n-dropdown>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NDropdown, NIcon, NInput } from 'naive-ui';
import { Bookmark, BookmarkRegular, Search } from '@vicons/fa';

@Component({
    components: {
        NDropdown,
        NIcon,
        NInput,

        Bookmark,
        BookmarkRegular,
        Search,
    },
    emits: [
        'busy',
        'touched',
        'update:entries'
    ],
})
class Filter extends Vue {
    @Prop({
        type: Object,
        required: true,
    })
    public filters!: any;

    @Prop({
        type: Object,
        required: true,
    })
    public options!: { [key: string]: IFilterOption };

    @Prop({
        required: true,
    })
    public entries!: Array<any>;

    @Prop({
        type: Function,
        required: true,
    })
    public searchFunction!: (filter: any) => any;

    @Prop({
        default: true,
        required: true,
        type: Boolean,
    })
    public loadInitial!: boolean;

    public search: string = '';
    public selectedKey: string = '';

    public filterOptions: { key: string; label: string }[] = [];
    public filterOptionsOrig: any = [];
    public showOptions: boolean = false;

    public bookmark: boolean = false;

    public created() {
        for (let key of Object.keys(this.options)) {
            let val = this.options[key];
            this.filterOptions.push({
                label: val.label,
                key: key,
            });
        }

        this.$watch(
            () => this.filters,
            () => {
                this.filterEntries();
            },
            { deep: true },
        );

        this.filterOptionsOrig = this.filterOptions;
    }

    public mounted() {
        if (this.loadInitial) {
            this.filterEntries();
        }
    }

    public filterSelected(key: string) {
        if (this.selectedKey && this.options[key]) {
            this.selectedKey = '';
            this.filterSelected(key);
        } else if (!this.selectedKey) {
            this.selectedKey = key;

            this.search = `${this.options[this.selectedKey].label}: `;

            const entry = this.options[this.selectedKey];
            if (!entry.options) {
                this.showOptions = false;
                (<any>this.$refs['filterInput']).focus();
            } else {
                this.filterOptions = [];

                for (let option of entry.options) {
                    if (this.filters[this.selectedKey]) {
                        let exists = this.filters[this.selectedKey].find(
                            (x: any) => x === option.key
                        );
                        if (exists) {
                            continue;
                        }
                    }

                    this.filterOptions.push(option)
                }
            }
        } else {
            if (this.options[this.selectedKey].multiple) {
                if (
                    this.filters[this.selectedKey] &&
                    this.filters[this.selectedKey].length > 0
                ) {
                    this.filters[this.selectedKey].push(key);
                } else {
                    this.filters[this.selectedKey] = [key];
                }
            } else {
                this.filters[this.selectedKey] = key;
            }

            this.filterEntries();
            this.reset();
        }
    }

    public handleKeydown(event: any) {
        if (event.keyCode === 8) {
            if (this.selectedKey && this.search.indexOf(':') === -1) {
                this.reset();
            }
        } else if (!this.selectedKey && event.keyCode === 13) {
            this.filters['name'] = this.search;
            this.reset();
        } else if (
            event.keyCode === 13 &&
            !this.options[this.selectedKey].options
        ) {
            this.filters[this.selectedKey] = this.search.replace(
                `${this.options[this.selectedKey].label}: `,
                '',
            );
            this.reset();
        }
    }

    public onClickOutside() {
        this.showOptions = false;
    }

    public createFilterBookmark() {
        this.bookmark = true;
        this.showOptions = false;
    }

    public handleShowDropdown() {
        if (this.bookmark) {
            this.showOptions = false;
            this.bookmark = false;
        } else {
            this.showOptions = true;
        }
    }

    private reset() {
        this.search = '';
        this.selectedKey = '';
        this.filterOptions = this.filterOptionsOrig;
        this.showOptions = false;
    }

    private filterEntries() {
        this.$emit('touched', true);

        let filters: { [key: string]: string } = {};

        for (const key in this.filters) {
            if (this.options[key].preRequest) {
                filters[key] = this.options[key].preRequest(
                    this.options[key],
                    this.filters[key],
                );
            } else if (this.options[key].multiple) {
                filters[key] = this.filters[key].join(',');
            } else {
                filters[key] = this.filters[key];
            }
        }

        this.$emit('busy', true);
        this.searchFunction(<any>filters)
            .then((x: any) => {
                const url = new URL(<any>window.location);
                url.search = '';
                for (let key of Object.keys(filters)) {
                    url.searchParams.set(key, filters[key]);
                }
                window.history.pushState({}, '', url);

                this.$emit('update:entries', x);
                this.$emit('busy', false);
            })
            .catch((e: any) => {
                this.$emit('busy', false);
                return [];
            });
    }
}

export interface IFilterOption {
    // label that is shown in the dropdown and the filter element
    label: string;

    // allows multiple entries of this type
    multiple?: boolean;

    // valid entries for the filter
    // if kept undefined it will assume a string input
    // otherwise it will show a dropdown with the given options
    options?: any[];
    // if options is an array of objects, the field that should be used as
    // filter needs to be set
    key?: string;

    // allows to transform the data before it is sent to the server
    //
    // filter: configuration for the filter
    // value: selected value
    preRequest?: (filter: IFilterOption, value: any) => string;
    // transforms the entry in-place, allows for overwriting the text shown,
    // without generating a new element
    //
    // filter: configuration for the filter
    // value: selected value
    transform?: (filter: IFilterOption, value: any) => string;
}

export default toNative(Filter);
</script>
