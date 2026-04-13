import { InputWrapper, SegmentedControl, Stack, Title } from "@mantine/core";
import { useEffect, useState, type ReactElement } from "react";

// can be used for initializing states
export const DEFAULT_GAS_BONUS: GasDecompression = 'TataraLvl5';
export const DEFAULT_MINERAL_BONUS: MineralCompression = 'NsTataraT2';

type StructureType = 'Athanor' | 'Tatara';
type SecurityType = 'Hs' | 'Ls' | 'Ns';
type RigType = 'NoRig' | 'T1' | 'T2';
type SkillLevelType = '0' | '1' | '2' | '3' | '4' | '5';

export type GasDecompression = 'AthanorLvl0' | 'AthanorLvl1' | 'AthanorLvl2' | 'AthanorLvl3' | 'AthanorLvl4' | 'AthanorLvl5' | 'TataraLvl0' | 'TataraLvl1' | 'TataraLvl2' | 'TataraLvl3' | 'TataraLvl4' | 'TataraLvl5';
export type MineralCompression = 'HsAthanorNoRig' | 'HsAthanorT1' | 'HsAthanorT2' | 'HsTataraNoRig' | 'HsTataraT1' | 'HsTataraT2' | 'LsAthanorNoRig' | 'LsAthanorT1' | 'LsAthanorT2' | 'LsTataraNoRig' | 'LsTataraT1' | 'LsTataraT2' | 'NsAthanorNoRig' | 'NsAthanorT1' | 'NsAthanorT2' | 'NsTataraNoRig' | 'NsTataraT1' | 'NsTataraT2';

export function CompressionMinimal({
    gasDecompression,
    mineralCompression,

    onGasUpdate,
    onMineralUpdate,
}: CompressionMinimalProp): ReactElement {
    const [mineralStructure, setMineralStructure] = useState<StructureType>('Tatara');
    const [mineralSecurity,  setMineralSecurity] = useState<SecurityType>('Ns');
    const [mineralRig,  setMineralRig] = useState<RigType>('T2');

    const [gasStructure, setGasStructure] = useState<StructureType>('Tatara');
    const [gasSkill, setGasSkill] = useState<SkillLevelType>('5');

    useEffect(() => {
        switch (gasDecompression) {
            case "AthanorLvl0":
                setGasStructure('Athanor');
                setGasSkill('0');
                break;
            case "AthanorLvl1":
                setGasStructure('Athanor');
                setGasSkill('1');
                break;
            case "AthanorLvl2":
                setGasStructure('Athanor');
                setGasSkill('2');
                break;
            case "AthanorLvl3":
                setGasStructure('Athanor');
                setGasSkill('3');
                break;
            case "AthanorLvl4":
                setGasStructure('Athanor');
                setGasSkill('4');
                break;
            case "AthanorLvl5":
                setGasStructure('Athanor');
                setGasSkill('5');
                break;

            case "TataraLvl0":
                setGasStructure('Tatara');
                setGasSkill('0');
                break;
            case "TataraLvl1":
                setGasStructure('Tatara');
                setGasSkill('1');
                break;
            case "TataraLvl2":
                setGasStructure('Tatara');
                setGasSkill('2');
                break;
            case "TataraLvl3":
                setGasStructure('Tatara');
                setGasSkill('3');
                break;
            case "TataraLvl4":
                setGasStructure('Tatara');
                setGasSkill('4');
                break;
            case "TataraLvl5":
                setGasStructure('Tatara');
                setGasSkill('5');
                break;

            default:
                setGasStructure('Tatara');
                setGasSkill('5');
        }
    }, [gasDecompression]);

    useEffect(() => {
        switch(mineralCompression) {
            case "HsAthanorNoRig":
                setMineralStructure('Athanor');
                setMineralSecurity('Hs');
                setMineralRig('NoRig');
                break;
            case "HsAthanorT1":
                setMineralStructure('Athanor');
                setMineralSecurity('Hs');
                setMineralRig('T1');
                break;
            case "HsAthanorT2":
                setMineralStructure('Athanor');
                setMineralSecurity('Hs');
                setMineralRig('T2');
                break;
            case "HsTataraNoRig":
                setMineralStructure('Tatara');
                setMineralSecurity('Hs');
                setMineralRig('NoRig');
                break;
            case "HsTataraT1":
                setMineralStructure('Tatara');
                setMineralSecurity('Hs');
                setMineralRig('T1');
                break;
            case "HsTataraT2":
                setMineralStructure('Tatara');
                setMineralSecurity('Hs');
                setMineralRig('T2');
                break;

            case "LsAthanorNoRig":
                setMineralStructure('Athanor');
                setMineralSecurity('Ls');
                setMineralRig('NoRig');
                break;
            case "LsAthanorT1":
                setMineralStructure('Athanor');
                setMineralSecurity('Ls');
                setMineralRig('T1');
                break;
            case "LsAthanorT2":
                setMineralStructure('Athanor');
                setMineralSecurity('Ls');
                setMineralRig('T2');
                break;
            case "LsTataraNoRig":
                setMineralStructure('Tatara');
                setMineralSecurity('Ls');
                setMineralRig('NoRig');
                break;
            case "LsTataraT1":
                setMineralStructure('Tatara');
                setMineralSecurity('Ls');
                setMineralRig('T1');
                break;
            case "LsTataraT2":
                setMineralStructure('Tatara');
                setMineralSecurity('Ls');
                setMineralRig('T2');
                break;

            case "NsAthanorNoRig":
                setMineralStructure('Athanor');
                setMineralSecurity('Ns');
                setMineralRig('NoRig');
                break;
            case "NsAthanorT1":
                setMineralStructure('Athanor');
                setMineralSecurity('Ns');
                setMineralRig('T1');
                break;
            case "NsAthanorT2":
                setMineralStructure('Athanor');
                setMineralSecurity('Ns');
                setMineralRig('T2');
                break;
            case "NsTataraNoRig":
                setMineralStructure('Tatara');
                setMineralSecurity('Ns');
                setMineralRig('NoRig');
                break;
            case "NsTataraT1":
                setMineralStructure('Tatara');
                setMineralSecurity('Ns');
                setMineralRig('T1');
                break;
            case "NsTataraT2":
                setMineralStructure('Tatara');
                setMineralSecurity('Ns');
                setMineralRig('T2');
                break;

            default:
                setMineralStructure('Tatara');
                setMineralSecurity('Ns');
                setMineralRig('T2');
        }
    }, [mineralCompression]);

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
                        { label: 'Highsec', value: 'Hs' },
                        { label: 'Lowsec', value: 'Ls' },
                        { label: 'Nullsec', value: 'Ns' },
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
    gasDecompression: GasDecompression;
    mineralCompression: MineralCompression;

    onGasUpdate(value: string): void;
    onMineralUpdate(value: string): void;
}
