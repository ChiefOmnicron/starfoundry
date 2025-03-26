import StructureUpdate from '@/structure/update/View.vue'

describe('Update', () => {
    it('should show an error, because updating the rigs failed', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures/bae12bba-df43-4a75-bb62-fa213c51a06f'
                },
                {
                    'id': 'bae12bba-df43-4a75-bb62-fa213c51a06f',
                    'name': 'My structure',
                    'system_id': 30004759,
                    'security': 'Nullsec/Wormhole',
                    'structure_type_id': 35827,
                    'rigs': [
                        // Standup XL-Set Structure and Component Manufacturing Efficiency II
                        43705,
                    ],
                    'services': [
                        // Standup Supercapital Shipyard I
                        35877,
                    ],
                    'structure_id': 0
                },
            )
            .as('getStructure');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/structures/bae12bba-df43-4a75-bb62-fa213c51a06f/permissions/is-owner'
            }, {
                statusCode: 200,
            })
            .as('getStructurePermission');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/search/systems?system_id=30004759'
                },
                [
                    {
                        'region_name': 'Delve',
                        'region_id': 10000060,
                        'system_name': '1DQ1-A',
                        'system_id': 30004759,
                        'security': -0.38578233
                    }
                ]
            )
            .as('getSystem');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/structures/35827/rigs'
                },
                []
            )
            .as('getRigs');

        cy.mount(<any>StructureUpdate, {
                props: {
                    structureId: 'bae12bba-df43-4a75-bb62-fa213c51a06f',
                }
            });

        cy.wait('@getStructure');
        cy.wait('@getStructurePermission');
        cy.wait('@getSystem');
        cy.wait('@getRigs');

        cy
            .get('[data-cy="structureUpdateTitle"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateGeneralInformation"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateInstalledRigs"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateInstalledServices"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateActions"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateDeleteObject"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureUpdateLoading"]')
            .should('not.exist');
        cy
            .get('[data-cy="structureUpdateNotFound"]')
            .should('not.exist');
        cy
            .get('[data-cy="structureUpdateError"]')
            .should('not.exist');
        cy
            .get('[data-cy="structureDeleteError"]')
            .should('not.exist');

        cy.intercept({
                    method: 'PUT',
                    url: '/api/v1/structures/bae12bba-df43-4a75-bb62-fa213c51a06f'
                },
                {
                    statusCode: 500,
                }
            )
            .as('update');

        cy
            .get('[data-cy^="clearSelectedRig"]')
            .filter(':not(:disabled)')
            .should('have.length', 1)
            .click();
        cy
            .get('[data-cy=structureUpdateSave]')
            .click();

        cy.wait('@update');

        cy
            .get('[data-cy="structureUpdateError"]')
            .should('be.visible');

        cy
            .get('[data-cy="structureUpdateGeneralInformation"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateInstalledRigs"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateInstalledServices"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateActions"]')
            .should('be.visible');
        cy
            .get('[data-cy="structureUpdateDeleteObject"]')
            .should('be.visible');
    });
});
