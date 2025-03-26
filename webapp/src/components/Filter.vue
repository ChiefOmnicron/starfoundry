<template>
    <n-dropdown
        :options="filterOptions"
        :render-label="renderLabel"
        :show="showOptions"
        @clickoutside="onClickOutside"
        @select="filterSelected"
        placement="bottom-start"
        trigger="click"
    >
        <n-input
            @click="showOptions = true"
            @keydown="handleKeydown"
            data-cy="filter-text"
            placeholder="#WithFilter"
            ref="filterInput"
            type="text"
            v-model:value="search"
        >
            <template #prefix>
                <n-icon>
                    <search />
                </n-icon>
            </template>
        </n-input>
    </n-dropdown>
</template>

<script lang="ts">
import { Component, Prop, Vue, toNative } from 'vue-facing-decorator';
import { NDropdown, NIcon, NInput, NSelect } from 'naive-ui';
import { Search } from '@vicons/fa';
import { type VNode } from 'vue';

@Component({
    components: {
        NDropdown,
        NIcon,
        NInput,
        NSelect,

        Search,
    },
    emits: [
        'busy',
        'touched',
        'update:entries',
    ],
})
class Filter extends Vue {
    @Prop({
        type:     Object,
        required: true
    })
    public filters!: any;

    @Prop({
        type:     Object,
        required: true
    })
    public options!: any;

    @Prop({
        required: true
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

    public filterOptions: { key: string, label: string }[] = [];
    public filterOptionsOrig: any = [];
    public showOptions: boolean = false;

    public entries_orig = this.entries;

    private translation!: any;

    public created() {
        for (let key of Object.keys(this.options)) {
            let val = this.options[key];
            this.filterOptions.push({
                label: val.label,
                key: key
            });
        }

        this.$watch(() => this.filters, () => {
            this.filterEntries();
        }, { deep: true });

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

                if (entry.name) {
                    entry.options.sort(
                        (a: any, b: any) => (a[entry.name] < b[entry.name] ? -1 : 1)
                    );
                }

                for (let option of entry.options) {
                    let key = option[entry.key] ? option[entry.key] : option;
                    let exists = this.filterOptions.find((x: any) => x.key === key);
                    if (exists) {
                        continue;
                    }

                    let label = option[entry.name] ? option[entry.name] : option;
                    this.filterOptions.push({
                        label,
                        key,
                    });
                }
            }
        } else {
            if (this.options[this.selectedKey].multiple) {
                if (this.filters[this.selectedKey] && this.filters[this.selectedKey].length > 0) {
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
        } else if (event.keyCode === 13 && !this.options[this.selectedKey].options) {
            this.filters[this.selectedKey] = this
                .search
                .replace(`${this.options[this.selectedKey].label}: `, '');
            this.reset();
        }
    }

    public renderLabel(x: any) {
        const entry = this.options[this.selectedKey];

        if (entry && entry.template) {
            return entry.template(x.key);
        } else {
            return x.label;
        }
    }

    public onClickOutside() {
        this.showOptions = false;
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
                filters[key] = this.options[key].preRequest(this.options[key], this.filters[key]);
            } else if (this.options[key].multiple) {
                filters[key] = this.filters[key].join(',');
            } else {
                filters[key] = this.filters[key];
            }
        }

        this.$emit('busy', true);
        this.searchFunction(<any>filters)
            .then((x: any) => {
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
    label:        string;

    // allows multiple entries of this type
    multiple?:    boolean;

    // valid entries for the filter
    // if kept undefined it will assume a string input
    // otherwise it will show a dropdown with the given options
    options?:      any[],
    // if options is an array of objects, the field name that should be used
    // as a name needs to be set
    name?:        string;
    // if options is an array of objects, the field that should be used as
    // filter needs to be set
    key?:         string;

    // allows to transform the data before it is sent to the server
    //
    // filter: configuration for the filter
    // value: selected value
    preRequest?: (filter: IFilterOption, value: any) => string;
    // allows to overwrite the template that is shown when a filter is selected
    //
    // filter: configuration for the filter
    // value: selected value
    template?:    (filter: IFilterOption, value: any) => VNode;
    // transforms the entry inplace, allows for overwriting the text shown,
    // without generating a new elememnt
    //
    // filter: configuration for the filter
    // value: selected value
    transform?:   (filter: IFilterOption, value: any) => string;

    item?:        boolean;
}

export default toNative(Filter);
</script>

