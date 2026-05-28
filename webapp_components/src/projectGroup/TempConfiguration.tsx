import { ItemList } from "@internal/list/ItemList";
import { Accordion, Text } from "@mantine/core";
import { JobSplittingRunList } from "./JobSplittingRunList";
import { BlueprintOverwriteList } from "./BlueprintOverwriteList";
import type { Item } from "@internal/services/item/model";
import type { BlueprintOverwrite } from "@internal/services/project-group/listDefaultBlueprintOverwrites";
import type { TypeId } from "@internal/services/utils";
import type { JobSplittingRun } from "@internal/services/project-group/listDefaultJobSplitting";
import type { Structure } from "@internal/services/structure/list";
import { StructureList } from "@internal/list/StructureList";

export function TempProjectGroupConfiguration({
    blacklist = [],
    showBlacklist = false,
    onBlacklistSelect = () => {},

    blueprintOverwrites = [],
    showBlueprintOverwrites = false,
    onBlueprintOverwriteDelete = () => {},
    onBlueprintOverwriteSelect = () => {},

    jobSplitting = [],
    showJobSplitting = false,
    onJobSplittingDelete = () => {},
    onJobSplittingSelect = () => {},

    markets = [],
    showMarket = false,
}: TempProjectGroupConfigurationProps) {
    const blacklistAccordion = showBlacklist
        ?   <Accordion.Item value='blacklist'>
                <Accordion.Control>
                    <Text>Blacklist</Text>

                    <Text size="sm" c="dimmed" fw={400}>
                        Configure items that should not be build.
                    </Text>
                </Accordion.Control>

                <Accordion.Panel>
                    <ItemList
                        selected={blacklist}
                        onSelect={onBlacklistSelect}
                        buildable
                        editable
                    />
                </Accordion.Panel>
            </Accordion.Item>
        :   <></>;

    const blueprintOverwriteAccordion = showBlueprintOverwrites
        ?   <Accordion.Item value='blueprint_overwrite'>
                <Accordion.Control>
                    <Text>Blueprint Overwrite</Text>

                    <Text size="sm" c="dimmed" fw={400}>
                        Overwrite the default Material Efficiency for blueprints.
                    </Text>
                </Accordion.Control>

                <Accordion.Panel>
                    <BlueprintOverwriteList
                        selected={blueprintOverwrites}
                        onDelete={onBlueprintOverwriteDelete}
                        onSelect={onBlueprintOverwriteSelect}
                        editable
                    />
                </Accordion.Panel>
            </Accordion.Item>
        :   <></>;

    const jobSplittingAccordion = showJobSplitting
        ?   <Accordion.Item value='job_splitting'>
                <Accordion.Control>
                    <Text>Job Splitting</Text>

                    <Text size="sm" c="dimmed" fw={400}>
                        Set the max runs for blueprints
                    </Text>
                </Accordion.Control>

                <Accordion.Panel>
                    <JobSplittingRunList
                        selected={jobSplitting}
                        onDelete={onJobSplittingDelete}
                        onSelect={onJobSplittingSelect}
                        editable
                    />
                </Accordion.Panel>
            </Accordion.Item>
        :   <></>;

    const marketAccordion = showMarket
        ?   <Accordion.Item value='market'>
                <Accordion.Control>
                    <Text>Markets</Text>

                    <Text size="sm" c="dimmed" fw={400}>
                        Markets that are used to calculate prices
                    </Text>
                </Accordion.Control>

                <Accordion.Panel>
                    <StructureList
                        structures={markets}

                        groupBySystem={false}
                        viewTarget='_blank'
                    />
                </Accordion.Panel>
            </Accordion.Item>
        :   <></>;

    return <>
        <Accordion chevronPosition="right" variant="contained">
            {blacklistAccordion}

            {blueprintOverwriteAccordion}

            {jobSplittingAccordion}

            {marketAccordion}
        </Accordion>
    </>
}

export type TempProjectGroupConfigurationProps = {
    blacklist?:                     Item[];
    showBlacklist?:                 boolean;
    onBlacklistSelect?:             (items: Item[]) => void;

    blueprintOverwrites?:           BlueprintOverwrite[],
    showBlueprintOverwrites?:       boolean;
    onBlueprintOverwriteDelete?:    (id: TypeId) => void;
    onBlueprintOverwriteSelect?:    (blueprintOverwrite: BlueprintOverwrite) => void;

    jobSplitting?:                  JobSplittingRun[];
    showJobSplitting?:              boolean;
    onJobSplittingDelete?:          (id: TypeId) => void;
    onJobSplittingSelect?:          (jobSplitRun: JobSplittingRun) => void;

    markets?:                       Structure[];
    showMarket?:                    boolean;
}
