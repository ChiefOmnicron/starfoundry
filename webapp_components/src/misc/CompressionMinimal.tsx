import { InputWrapper, SegmentedControl, Stack, Title } from "@mantine/core";
import { useState, type ReactElement } from "react";

// can be used for initializing states
export const DEFAULT_GAS_BONUS: string = 'TataraLvl5';
export const DEFAULT_MINERAL_BONUS: string = 'NsTataraT2';

type StructureType = 'Athanor' | 'Tatara';
type SecurityType = 'HS' | 'LS' | 'NS';
type RigType = 'NoRig' | 'T1' | 'T2';
type SkillLevelType = '0' | '1' | '2' | '3' | '4' | '5';

export function CompressionMinimal({
    onGasUpdate,
    onMineralUpdate,
}: CompressionMinimalProp): ReactElement {
    const [mineralStructure, setMineralStructure] = useState<StructureType>('Tatara');
    const [mineralSecurity,  setMineralSecurity] = useState<SecurityType>('NS');
    const [mineralRig,  setMineralRig] = useState<RigType>('T2');

    const [gasStructure, setGasStructure] = useState<StructureType>('Tatara');
    const [gasSkill, setGasSkill] = useState<SkillLevelType>('5');

    const mineralCode = (
        structure:  StructureType,
        security:   SecurityType,
        rig:        RigType,
    ) => {
        onMineralUpdate(`${security}${structure}${rig}`);
    }

    const gasCode = (
        structure:  StructureType,
        skill:      '0' | '1' | '2' | '3' | '4' | '5',
    ) => {
        onGasUpdate(`${structure}Lvl${skill}`);
    }

    return <>
        <Title order={2}>Mineral Compression</Title>
        <Stack>
            <InputWrapper
                label="Structure"
                description="Select a structure"
            >
                <SegmentedControl
                    value={gasStructure}
                    onChange={(v) => {
                        setMineralStructure(v as StructureType);
                        mineralCode(v as StructureType, mineralSecurity, mineralRig);
                    }}
                    data={[
                        { label: 'Athanor', value: 'Athanor' },
                        { label: 'Tatara', value: 'Tatara' },
                    ]}
                    fullWidth
                />
            </InputWrapper>

            <InputWrapper
                label="Security"
                description="Select system security"
            >
                <SegmentedControl
                    value={mineralSecurity}
                    onChange={(v) => {
                        setMineralSecurity(v as SecurityType);
                        mineralCode(mineralStructure, v as SecurityType, mineralRig);
                    }}
                    data={[
                        { label: 'Highsec', value: 'HS' },
                        { label: 'Lowsec', value: 'LS' },
                        { label: 'Nullsec', value: 'NS' },
                    ]}
                    fullWidth
                />
            </InputWrapper>

            <InputWrapper
                label="Skill"
                description="Select the Rig"
            >
                <SegmentedControl
                    value={mineralRig}
                    onChange={(v) => {
                        setMineralRig(v as RigType);
                        mineralCode(mineralStructure, mineralSecurity, v as RigType);
                    }}
                    data={[
                        { label: 'No Rig', value: 'NoRig' },
                        { label: 'T1 Rig', value: 'T1' },
                        { label: 'T2 Rig', value: 'T2' },
                    ]}
                    fullWidth
                />
            </InputWrapper>

            <Title order={2}>Gas Decompression </Title>
                <InputWrapper
                    label="Structure"
                    description="Select a structure"
                >
                    <SegmentedControl
                        value={gasStructure}
                        onChange={(v) => {
                            setGasStructure(v as StructureType);
                            gasCode(v as StructureType, gasSkill);
                        }}
                        data={[
                            { label: 'Athanor', value: 'Athanor' },
                            { label: 'Tatara', value: 'Tatara' },
                        ]}
                        fullWidth
                    />
                </InputWrapper>

                <InputWrapper
                    label="Skill"
                    description="Select the skill level"
                >
                    <SegmentedControl
                        value={gasSkill}
                        onChange={(v) => {
                            setGasSkill(v as SkillLevelType);
                            gasCode(gasStructure, v as SkillLevelType);
                        }}
                        data={[
                            { label: 'Level 0', value: '0' },
                            { label: 'Level 1', value: '1' },
                            { label: 'Level 2', value: '2' },
                            { label: 'Level 3', value: '3' },
                            { label: 'Level 4', value: '4' },
                            { label: 'Level 5', value: '5' },
                        ]}
                        fullWidth
                    />
                </InputWrapper>
        </Stack>
    </>
}

export type CompressionMinimalProp = {
    onGasUpdate(value: string): void;
    onMineralUpdate(value: string): void;
}
