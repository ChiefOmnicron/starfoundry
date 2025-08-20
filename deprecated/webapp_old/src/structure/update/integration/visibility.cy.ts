import StructureUpdate from '@/structure/update/View.vue';

describe('Visibility', () => {
    it('should show the spinner', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures/4c29acc7-23f4-465c-8813-78ce9417953e',
            },
            (req) => {
                // do nothing with the req, only call the response with a 10s delay.
                req.continue((res) => {
                    res.delay = 10000;
                    res.send();
                });
            },
        ).as('getStructure');

        cy.mount(<any>StructureUpdate, {
            props: {
                structureId: '4c29acc7-23f4-465c-8813-78ce9417953e',
            },
        });

        cy.get('[data-cy="structureUpdateLoading"]').should('be.visible');

        cy.get('[data-cy="structureUpdateTitle"]').should('not.exist');
        cy.get('[data-cy="structureUpdateNotFound"]').should('not.exist');
        cy.get('[data-cy="structureUpdateUpdating"]').should('not.exist');
        cy.get('[data-cy="structureUpdateGeneralInformation"]').should(
            'not.exist',
        );
        cy.get('[data-cy="structureUpdateInstalledRigs"]').should('not.exist');
        cy.get('[data-cy="structureUpdateInstalledServices"]').should(
            'not.exist',
        );
        cy.get('[data-cy="structureUpdateActions"]').should('not.exist');
        cy.get('[data-cy="structureUpdateDeleteObject"]').should('not.exist');
        cy.get('[data-cy="structureDeleteError"]').should('not.exist');
    });

    it('should show the not found error', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures/8855c39a-1ebb-413c-9802-1f9fad3bef2d',
            },
            {
                statusCode: 404,
            },
        ).as('getStructure');

        cy.mount(<any>StructureUpdate, {
            props: {
                structureId: '8855c39a-1ebb-413c-9802-1f9fad3bef2d',
            },
        });

        cy.wait('@getStructure');

        cy.get('[data-cy="structureUpdateNotFound"]').should('be.visible');

        cy.get('[data-cy="structureUpdateLoading"]').should('not.exist');
        cy.get('[data-cy="structureUpdateTitle"]').should('not.exist');
        cy.get('[data-cy="structureUpdateError"]').should('not.exist');
        cy.get('[data-cy="structureUpdateGeneralInformation"]').should(
            'not.exist',
        );
        cy.get('[data-cy="structureUpdateInstalledRigs"]').should('not.exist');
        cy.get('[data-cy="structureUpdateInstalledServices"]').should(
            'not.exist',
        );
        cy.get('[data-cy="structureUpdateActions"]').should('not.exist');
        cy.get('[data-cy="structureUpdateDeleteObject"]').should('not.exist');
        cy.get('[data-cy="structureDeleteError"]').should('not.exist');
    });

    it('should show everything', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures/a331940e-6669-4502-b5cd-18f2db1679e7',
            },
            {
                id: 'a331940e-6669-4502-b5cd-18f2db1679e7',
                name: 'My structure',
                system_id: 30004759,
                security: 'Nullsec/Wormhole',
                structure_type_id: 35836,
                rigs: [],
                services: [],
                structure_id: 0,
            },
        ).as('getStructure');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/search/systems?system_id=30004759',
            },
            [
                {
                    region_name: 'Delve',
                    region_id: 10000060,
                    system_name: '1DQ1-A',
                    system_id: 30004759,
                    security: -0.38578233,
                },
            ],
        ).as('getSystem');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures/a331940e-6669-4502-b5cd-18f2db1679e7/permissions/is-owner',
            },
            {
                statusCode: 200,
            },
        ).as('getStructurePermission');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures/35836/rigs',
            },
            [],
        ).as('getRigs');

        cy.mount(<any>StructureUpdate, {
            props: {
                structureId: 'a331940e-6669-4502-b5cd-18f2db1679e7',
            },
        });

        cy.wait('@getStructure');
        cy.wait('@getSystem');
        cy.wait('@getRigs');
        cy.wait('@getStructurePermission');

        cy.get('[data-cy="structureUpdateTitle"]').should('be.visible');
        cy.get('[data-cy="structureUpdateGeneralInformation"]').should(
            'be.visible',
        );
        cy.get('[data-cy="structureUpdateInstalledRigs"]').should('be.visible');
        cy.get('[data-cy="structureUpdateInstalledServices"]').should(
            'be.visible',
        );
        cy.get('[data-cy="structureUpdateActions"]').should('be.visible');
        cy.get('[data-cy="structureUpdateDeleteObject"]').should('be.visible');

        cy.get('[data-cy="structureUpdateLoading"]').should('not.exist');
        cy.get('[data-cy="structureUpdateNotFound"]').should('not.exist');
        cy.get('[data-cy="structureUpdateUpdating"]').should('not.exist');
        cy.get('[data-cy="structureDeleteError"]').should('not.exist');
    });
});
