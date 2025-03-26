import StructureCreate from '@/structure/create/View.vue'

describe('Visibility', () => {
    it('should show everything', () => {
        cy.intercept({
                method: 'GET',
                url: '/api/v1/project-groups?projects=READ,WRITE&structures=WRITE'
            },
            [
                '29993589-647d-4c21-9daa-ae6a15fedb1a',
            ])
            .as('getWritableStructureGroups');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/project-groups/29993589-647d-4c21-9daa-ae6a15fedb1a'
            },
            {
                "name": "Example group",
                "description": "Test",
                "owner": 0,
                "owner_name": "Nobody",
                "is_owner": true
            })
            .as('getStructureGroups');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/search/systems'
                }, []
            )
            .as('searchSystems');

        cy.mount(<any>StructureCreate, {});

        cy.wait('@getWritableStructureGroups');
        cy.wait('@getStructureGroups');
        cy.wait('@searchSystems');

        cy.get('[data-cy="structureCreateTitle"]').should('be.visible');
        cy.get('[data-cy="strucutreCreateGeneralInformation"]').should('be.visible');
        cy.get('[data-cy="structureCreateActions"]').should('be.visible');

        cy.get('[data-cy="structureCreateButton"]').should('be.disabled');

        cy.get('[data-cy="structureCreateInstalledRigs"]').should('not.exist');
        cy.get('[data-cy="structureCreateInstalledServices"]').should('not.exist');
        cy.get('[data-cy="strucutreCreateResolveStructureError"]').should('not.exist');
        cy.get('[data-cy="strucutreCreateUnexpectedError"]').should('not.exist');
    });

    it('should show the structure group load error', () => {
        cy.intercept({
                method: 'GET',
                url: '/api/v1/project-groups?projects=READ,WRITE&structures=WRITE'
            },
            {
                statusCode: 500
            })
            .as('getStructureGroups');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/search/systems'
                }, []
            )
            .as('searchSystems');

        cy.mount(<any>StructureCreate, {});

        cy.wait('@getStructureGroups');
        cy.wait('@searchSystems');

        cy.get('[data-cy="structureCreateTitle"]').should('be.visible');
        cy.get('[data-cy="strucutreCreateGeneralInformation"]').should('be.visible');
        cy.get('[data-cy="structureCreateActions"]').should('be.visible');

        cy.get('[data-cy="structureCreateButton"]').should('be.disabled');

        cy.get('[data-cy="structureCreateInstalledRigs"]').should('not.exist');
        cy.get('[data-cy="structureCreateInstalledServices"]').should('not.exist');
        cy.get('[data-cy="strucutreCreateResolveStructureError"]').should('not.exist');
        cy.get('[data-cy="strucutreCreateUnexpectedError"]').should('not.exist');
    });

    it('should show the structure resolve error', () => {
        cy.intercept({
                method: 'GET',
                url: '/api/v1/project-groups?projects=READ,WRITE&structures=WRITE'
            },
            [
                '29993589-647d-4c21-9daa-ae6a15fedb1a',
            ])
            .as('getWritableStructureGroups');
        cy.intercept({
                method: 'GET',
                url: '/api/v1/project-groups/29993589-647d-4c21-9daa-ae6a15fedb1a'
            },
            {
                "name": "Example group",
                "description": "Test",
                "owner": 0,
                "owner_name": "Nobody",
                "is_owner": true
            })
            .as('getStructureGroups');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/search/systems'
                }, []
            )
            .as('searchSystems');

        cy.mount(<any>StructureCreate, {});

        cy.wait('@getWritableStructureGroups');
        cy.wait('@getStructureGroups');
        cy.wait('@searchSystems');

        cy.intercept({
                method: 'GET',
                url: '/api/v1/structures/1/resolve'
            },
            {
                statusCode: 400
            })
            .as('resolveStructure');

        cy.get('[data-cy="StructureResolveInput"]').type('1');
        cy.get('[data-cy="StructureResolveButton"]').click();
        cy.wait('@resolveStructure');

        cy.get('[data-cy="structureCreateTitle"]').should('be.visible');
        cy.get('[data-cy="strucutreCreateGeneralInformation"]').should('be.visible');
        cy.get('[data-cy="structureCreateActions"]').should('be.visible');
        cy.get('[data-cy="strucutreCreateResolveStructureError"]').should('be.visible');

        cy.get('[data-cy="structureCreateButton"]').should('be.disabled');

        cy.get('[data-cy="structureCreateInstalledRigs"]').should('not.exist');
        cy.get('[data-cy="structureCreateInstalledServices"]').should('not.exist');
        cy.get('[data-cy="strucutreCreateUnexpectedError"]').should('not.exist');
    });
});
