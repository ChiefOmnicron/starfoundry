import StructureOverview from '@/structure/overview/View.vue'

describe('Visibility', () => {
    it('should show the spinner', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures'
                },
                req => {
                    // do nothing with the req, only call the response with a 10s delay.
                    req.continue(res => {
                    res.delay = 10000;
                    res.send();
                    });
                },
            )
            .as('getEmptyStructures');

        cy.mount(<any>StructureOverview, {});

        cy
            .get('[data-cy="structuresLoaderInitial"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureLoadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresFilter"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresActionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresEmptyNoSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresDataTable"]')
            .should('not.exist');
    });

    it('should show no structures and have the option to create one', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures'
                },
                []
            )
            .as('getEmptyStructures');

        cy.mount(<any>StructureOverview, {
        });

        cy.wait('@getEmptyStructures');

        cy
            .get('[data-cy="structuresNoEntries"]')
            .should('be.visible');
        cy
            .get('[data-cy="structuresNoEntriesAddButton"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureLoadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresFilter"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresActionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresEmptyNoSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresDataTable"]')
            .should('not.exist');
    });

    it('should show the initial loading error message', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures'
                },
                {
                    statusCode: 500,
                }
            )
            .as('getEmptyStructures');

        cy.mount(<any>StructureOverview, {});

        cy.wait('@getEmptyStructures');

        cy
            .get('[data-cy="structureLoadingError"]')
            .should('be.visible');

        cy
            .get('[data-cy="structuresNoEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresFilter"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresActionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresEmptyNoSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresDataTable"]')
            .should('not.exist');
    });

    it('should show one item', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures'
                },
                [
                    '6e63d58f-f291-435a-bc35-82c8b47d5527'
                ]
            )
            .as('listStructures');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures/6e63d58f-f291-435a-bc35-82c8b47d5527'
                },
                {
                    name:              'Test Structure',
                    rigs:              [],
                    security:          'Nullsec/Wormhole',
                    services:          [],
                    structure_id:      0,
                    structure_type_id: 35834,
                    system_id:         30004759,
                    system_name:       '1DQ1-A',
                    project_group_ids: [],
                }
            )
            .as('getOneStructure');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/search/systems?system_id=30004759'
            }, [{
                region_name: 'Delve',
                region_id:   10000060,
                system_name: '1DQ1-A',
                system_id:   30004759,
                security:    'Nullsec/Wormhole',
            }])
            .as('getSearchSystem');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/items/resolve/ids/35834'
            }, {
                type_id:     35834,
                category_id: 65,
                group_id:    1657,
                volume:      800000,
                name:        "Keepstar",
                base_price:  null,
            })
            .as('getItemName');

        cy.mount(<any>StructureOverview, {
        });

        cy.wait('@listStructures');
        cy.wait('@getOneStructure');
        cy.wait('@getSearchSystem');
        cy.wait('@getItemName');

        cy
            .get('[data-cy="structuresFilter"]')
            .should('be.visible');
        cy
            .get('[data-cy="structuresActionGroup"]')
            .should('be.visible');
        cy
            .get('[data-cy="structuresDataTable"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureLoadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresEmptyNoSearchResult"]')
            .should('not.exist');
    });

    it('should filter and show one item', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures'
                },
                [
                    '6e63d58f-f291-435a-bc35-82c8b47d5527'
                ]
            )
            .as('listStructures');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures/6e63d58f-f291-435a-bc35-82c8b47d5527'
                },
                {
                    name:              'Test Structure',
                    rigs:              [],
                    security:          'Nullsec/Wormhole',
                    services:          [],
                    structure_id:      0,
                    structure_type_id: 35834,
                    system_id:         30004759,
                    system_name:       '1DQ1-A',
                    project_group_ids: [],
                }
            )
            .as('getOneStructure');

        cy.intercept({
                method: 'GET',
                url: '/api/v1/structures/6e63d58f-f291-435a-bc35-82c8b47d5527/permission'
            },
            {
                structure_owner: true,
            })
            .as('getPermission');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/search/systems?system_id=30004759'
            }, [{
                region_name: 'Delve',
                region_id:   10000060,

                system_name: '1DQ1-A',
                system_id:   30004759,
                security:    'Nullsec/Wormhole',
            }])
            .as('getSearchSystem');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/items/resolve/ids/35834'
            }, {
                type_id:     35834,
                category_id: 65,
                group_id:    1657,
                volume:      800000,
                name:        "Keepstar",
                base_price:  null,
            })
            .as('getItemName');

        cy.mount(<any>StructureOverview, {
        });

        cy.wait('@listStructures');
        cy.wait('@getOneStructure');
        cy.wait('@getSearchSystem');
        cy.wait('@getItemName');

        cy.intercept({
                method: 'GET',
                url: '/api/v1/structures?name=Test'
            },
            [
                '6e63d58f-f291-435a-bc35-82c8b47d5527'
            ])
            .as('filterActive');

        cy.get('[data-cy="filter-text"]')
            .type('Test')
            .type('{enter}');

        cy.wait('@filterActive');
        cy.wait('@getOneStructure');
        cy.wait('@getSearchSystem');

        cy
            .get('[data-cy="structuresFilter"]')
            .should('be.visible');
        cy
            .get('[data-cy="structuresActionGroup"]')
            .should('be.visible');
        cy
            .get('[data-cy="structuresDataTable"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureLoadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresNoEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresLoaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="structuresEmptyNoSearchResult"]')
            .should('not.exist');
    });
});
