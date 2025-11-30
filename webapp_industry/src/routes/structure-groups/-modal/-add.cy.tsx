import { createRootRoute, createRouter, RouterProvider } from "@tanstack/react-router";
import { MantineProvider } from "@mantine/core";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AddStructure } from '@/routes/structures/-modal/add';

import '@mantine/core/styles.css';

const queryClient = new QueryClient({
    defaultOptions: {
        dehydrate: {
            shouldDehydrateQuery: () => false
        },
        queries: {
            retry: false,
        }
    }
});

const componentMount = () => {
    const routeTree = createRootRoute({
        component: () => <AddStructure close={() => {}} />,
    });
    const router = createRouter({
        routeTree,
        defaultStaleTime: 0,
        scrollRestoration: true,
    });
    return(
        <MantineProvider>
            <QueryClientProvider client={queryClient}>
                <RouterProvider router={router} />
            </QueryClientProvider>
        </MantineProvider>
    );
}

describe('Add structure', () => {
    it('happy path of adding a structure', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/auth/token',
            },
            (req) => {
                req.reply({
                    body: {
                        access_token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.KMUFsIDTnFmyG3nMiGM6H9FNFUROf3wh7SmqJp-QV30'
                    },
                    statusCode: 200,
                });
            },
        ).as('AuthToken');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/eve/structures/1000000000000',
            },
            (req) => {
                req.reply({
                    body: TEST_DATA,
                    delay: 1000,
                    statusCode: 200,
                });
            },
        ).as('ResolveStructure');
        cy.intercept(
            {
                method: 'POST',
                url: '/api/structures',
            },
            (req) => {
                req.reply({
                    body: {
                        id: '019a1bdf-37f9-7f7a-8e10-c99f3eb9b674'
                    },
                });
            },
        ).as('ResolveStructure');

        cy.mount(componentMount());

        cy.get('[data-cy="infoTable"]').should('not.exist');
        cy.get('[data-cy="rigSelector"]').should('not.exist');
        cy.get('[data-cy="serviceSelector"]').should('not.exist');
        cy.get('[data-cy="addStructure"]').should('not.exist');
        cy.get('[data-cy="closeStructure"').should('be.visible');

        cy.get('[data-cy="structureId"]').type('Some Character > <url=showinfo:35834//1000000000000>SomeSystem - Some Structure</url>');
        cy.get('[data-cy="resolveStructure"').click();

        cy.get('[data-cy="resolveStructure"').should('be.disabled');
        cy.get('[data-cy="resolveStructure"').should('have.attr', 'data-loading');

        cy.wait('@AuthToken');
        cy.wait('@ResolveStructure');

        cy.get('[data-cy="infoTable"]').should('be.visible');
        cy.get('[data-cy="rigSelector"]').should('be.visible');
        cy.get('[data-cy="serviceSelector"]').should('be.visible');
        cy.get('[data-cy="addStructure"]').should('be.visible');
        cy.get('[data-cy="closeStructure"').should('be.visible');

        cy.get('[data-cy="addStructure"]').click();
    });
});

const TEST_DATA = {
    "structure_id": 1000000000000,
    "name": "SomeSystem - SomeStructure",
    "system": {
        "region_id": 10000000,
        "region_name": "SomeRegion",
        "constellation_id": 20000000,
        "constellation_name": "SomeConstellation",
        "system_id": 30000000,
        "system_name": "SomeSystem",
        "security": -0.186762,
        "security_str": "NULLSEC"
    },
    "item": {
        "type_id": 35834,
        "category": {
            "category_id": 65,
            "name": "Structure"
        },
        "group": {
            "group_id": 1657,
            "category_id": 65,
            "name": "Citadel"
        },
        "volume": 800000.0,
        "name": "Keepstar",
        "meta_group_id": 1,
        "repackaged": null
    },
    "position": {
        "x": 1643733600000.0,
        "y": -76999926000.0,
        "z": -262765690000.0
    },
    "rigs": [
        {
            "item": {
                "type_id": 37180,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1868,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Ship Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Ship Manufacturing Efficiency I",
                "meta_group_id": 54,
                "repackaged": null
            },
            "excludes": [
                37181
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 37179,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1867,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Equipment and Consumable Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Equipment and Consumable Manufacturing Efficiency II",
                "meta_group_id": 53,
                "repackaged": null
            },
            "excludes": [
                37178
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 37182,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1870,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Laboratory Optimization"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Laboratory Optimization II",
                "meta_group_id": 53,
                "repackaged": null
            },
            "excludes": [
                37183
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 46642,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1945,
                    "category_id": 66,
                    "name": "Structure Resource Rig XL - Reprocessing"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Reprocessing Monitor II",
                "meta_group_id": 53,
                "repackaged": null
            },
            "excludes": [
                46641
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 37181,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1868,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Ship Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Ship Manufacturing Efficiency II",
                "meta_group_id": 53,
                "repackaged": null
            },
            "excludes": [
                37180
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 43705,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1869,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Structure and Component Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Structure and Component Manufacturing Efficiency II",
                "meta_group_id": 53,
                "repackaged": null
            },
            "excludes": [
                43704
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 45548,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1869,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Structure and Component Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Thukker Structure and Component Manufacturing Efficiency",
                "meta_group_id": 52,
                "repackaged": null
            },
            "excludes": [],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 43704,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1869,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Structure and Component Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Structure and Component Manufacturing Efficiency I",
                "meta_group_id": 54,
                "repackaged": null
            },
            "excludes": [
                43705
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 37183,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1870,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Laboratory Optimization"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Laboratory Optimization I",
                "meta_group_id": 54,
                "repackaged": null
            },
            "excludes": [
                37182
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 46641,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1945,
                    "category_id": 66,
                    "name": "Structure Resource Rig XL - Reprocessing"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Reprocessing Monitor I",
                "meta_group_id": 54,
                "repackaged": null
            },
            "excludes": [
                46642
            ],
            "material": null,
            "time": null,
            "category_groups": []
        },
        {
            "item": {
                "type_id": 37178,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1867,
                    "category_id": 66,
                    "name": "Structure Engineering Rig XL - Equipment and Consumable Efficiency"
                },
                "volume": 40.0,
                "name": "Standup XL-Set Equipment and Consumable Manufacturing Efficiency I",
                "meta_group_id": 54,
                "repackaged": null
            },
            "excludes": [
                37179
            ],
            "material": null,
            "time": null,
            "category_groups": []
        }
    ],
    "services": {
        "services": [
            {
                "type_id": 35881,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1415,
                    "category_id": 66,
                    "name": "Structure Engineering Service Module"
                },
                "volume": 32000.0,
                "name": "Standup Capital Shipyard I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 35894,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1321,
                    "category_id": 66,
                    "name": "Structure Citadel Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Cloning Center I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 45550,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1415,
                    "category_id": 66,
                    "name": "Structure Engineering Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Hyasyoda Research Lab",
                "meta_group_id": 52,
                "repackaged": null
            },
            {
                "type_id": 35886,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1415,
                    "category_id": 66,
                    "name": "Structure Engineering Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Invention Lab I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 35878,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1415,
                    "category_id": 66,
                    "name": "Structure Engineering Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Manufacturing Plant I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 35892,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1321,
                    "category_id": 66,
                    "name": "Structure Citadel Service Module"
                },
                "volume": 32000.0,
                "name": "Standup Market Hub I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 35899,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1322,
                    "category_id": 66,
                    "name": "Structure Resource Processing Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Reprocessing Facility I",
                "meta_group_id": 54,
                "repackaged": null
            },
            {
                "type_id": 35891,
                "category": {
                    "category_id": 66,
                    "name": "Structure Module"
                },
                "group": {
                    "group_id": 1415,
                    "category_id": 66,
                    "name": "Structure Engineering Service Module"
                },
                "volume": 4000.0,
                "name": "Standup Research Lab I",
                "meta_group_id": 54,
                "repackaged": null
            }
        ],
        "slots": 7
    }
}
