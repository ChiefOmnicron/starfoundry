import StatusTag from './StatusTag.vue';

describe('StatusTag props', () => {
    it('should show preparing', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'PREPARING',
            },
        });

        cy.get('[data-cy="preparing"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(51, 54, 57)')
            .contains('Preparing');
        cy.get('[data-cy="cancelled"]').should('not.exist');
        cy.get('[data-cy="paused"]').should('not.exist');
        cy.get('[data-cy="done"]').should('not.exist');
        cy.get('[data-cy="in_progress"]').should('not.exist');
        cy.get('[data-cy="default"]').should('not.exist');
    });

    it('should show cancelled', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'CANCELLED',
            },
        });

        cy.get('[data-cy="preparing"]').should('not.exist');
        cy.get('[data-cy="cancelled"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(208, 48, 80)')
            .contains('Cancelled');
        cy.get('[data-cy="paused"]').should('not.exist');
        cy.get('[data-cy="done"]').should('not.exist');
        cy.get('[data-cy="in_progress"]').should('not.exist');
        cy.get('[data-cy="default"]').should('not.exist');
    });

    it('should show paused', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'PAUSED',
            },
        });

        cy.get('[data-cy="preparing"]').should('not.exist');
        cy.get('[data-cy="cancelled"]').should('not.exist');
        cy.get('[data-cy="paused"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(240, 160, 32)')
            .contains('Paused');
        cy.get('[data-cy="done"]').should('not.exist');
        cy.get('[data-cy="in_progress"]').should('not.exist');
        cy.get('[data-cy="default"]').should('not.exist');
    });

    it('should show done', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'DONE',
            },
        });

        cy.get('[data-cy="preparing"]').should('not.exist');
        cy.get('[data-cy="cancelled"]').should('not.exist');
        cy.get('[data-cy="paused"]').should('not.exist');
        cy.get('[data-cy="done"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(24, 160, 88)')
            .contains('Done');
        cy.get('[data-cy="in_progress"]').should('not.exist');
        cy.get('[data-cy="default"]').should('not.exist');
    });

    it('should show in progress', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'IN_PROGRESS',
            },
        });

        cy.get('[data-cy="preparing"]').should('not.exist');
        cy.get('[data-cy="cancelled"]').should('not.exist');
        cy.get('[data-cy="paused"]').should('not.exist');
        cy.get('[data-cy="done"]').should('not.exist');
        cy.get('[data-cy="in_progress"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(32, 128, 240)')
            .contains('In Progress');
        cy.get('[data-cy="default"]').should('not.exist');
    });

    it('should show invalid', () => {
        cy.mount(<any>StatusTag, {
            props: {
                status: 'INVALID',
            },
        });

        cy.get('[data-cy="preparing"]').should('not.exist');
        cy.get('[data-cy="cancelled"]').should('not.exist');
        cy.get('[data-cy="paused"]').should('not.exist');
        cy.get('[data-cy="done"]').should('not.exist');
        cy.get('[data-cy="in_progress"]').should('not.exist');
        cy.get('[data-cy="default"]')
            .should('be.visible')
            .should('have.css', 'color', 'rgb(208, 48, 80)')
            .contains('Invalid Status');
    });
});
