import { h } from 'vue';
import Card from '@/components/Card.vue';

describe('Card props', () => {
    it('with title', () => {
        cy.mount(<any>Card, {
            props: {
                title: 'This is a header',
            },
        });

        cy.get('[data-cy="header"]')
            .should('be.visible')
            .contains('This is a header');
        cy.get('[data-cy="subheader"]').should('not.exist');
    });

    it('with subtitle', () => {
        cy.mount(<any>Card, {
            props: {
                subtitle: 'This is a subheader',
            },
        });

        cy.get('[data-cy="subheader"]')
            .should('be.visible')
            .contains('This is a subheader');
        cy.get('[data-cy="header"]').should('not.exist');
    });

    it('no title', () => {
        cy.mount(<any>Card, {
            props: {
                noTitle: true,
            },
        });

        cy.get('[data-cy="header"]').should('not.exist');

        cy.get('[data-cy="subheader"]').should('not.exist');
    });

    it('prefer title over subtitle', () => {
        cy.mount(<any>Card, {
            props: {
                title: 'Title',
                subtitle: 'Subtitle',
            },
        });

        cy.get('[data-cy="header"]').should('be.visible').contains('Title');

        cy.get('[data-cy="subheader"]').should('not.exist');
    });

    it('the card has a red border', () => {
        cy.mount(<any>Card, {
            props: {
                danger: true,
                title: 'Dangerous stuff happening here',
            },
        });

        cy.get('[data-cy="card"]').should('have.css', 'border');
    });
});

describe('Card slots', () => {
    it('default slot', () => {
        cy.mount(<any>Card, {
            props: {
                noTitle: true,
            },
            slots: {
                default: h('div', { id: 'inner-content' }, 'Hello World'),
            },
        });

        cy.get('#inner-content').contains('Hello World');
    });
});
