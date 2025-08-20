import { h } from 'vue';
import { NNotificationProvider } from 'naive-ui';
import StockBlueprintList from '@/stock/blueprint/list/View.vue';

const mount = <any>h(NNotificationProvider, {}, [h(StockBlueprintList)]);

describe('Visibility', () => {
    it('should show the spinner', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/structures',
            },
            (req) => {
                // do nothing with the req, only call the response with a 10s delay.
                req.continue((res) => {
                    res.delay = 10000;
                    res.send();
                });
            },
        ).as('getEmptyStructures');

        cy.mount(mount);

        cy.get('[data-cy="stockBlueprintLoaderInitial"]').should('be.visible');

        cy.get('[data-cy="stockBlueprintLoadingError"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntries"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintFilter"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintActionGroup"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderApi"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintEmptyNoSearchResult"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintDataTable"]').should('not.exist');
    });

    it('should show no structures and have the option to create one', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints',
            },
            [],
        ).as('getEmptyBlueprintStock');

        cy.mount(mount);

        cy.wait('@getEmptyBlueprintStock');

        cy.get('[data-cy="stockBlueprintNoEntries"]').should('be.visible');
        cy.get('[data-cy="stockBlueprintNoEntriesAddButton"]').should(
            'be.visible',
        );

        cy.get('[data-cy="stockBlueprintLoadingError"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderInitial"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintFilter"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintActionGroup"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderApi"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintEmptyNoSearchResult"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintDataTable"]').should('not.exist');
    });

    it('should show the initial loading error message', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints',
            },
            {
                statusCode: 500,
            },
        ).as('getBlueprintStockError');

        cy.mount(mount);

        cy.wait('@getBlueprintStockError');

        cy.get('[data-cy="stockBlueprintLoadingError"]').should('be.visible');

        cy.get('[data-cy="stockBlueprintNoEntries"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntriesAddButton"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintLoaderInitial"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintFilter"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintActionGroup"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderApi"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintEmptyNoSearchResult"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintDataTable"]').should('not.exist');
    });

    it('should show one item', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints',
            },
            ['6e63d58f-f291-435a-bc35-82c8b47d5527'],
        ).as('listStocks');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints/6e63d58f-f291-435a-bc35-82c8b47d5527',
            },
            {
                name: 'Test BPC Stock',
                description: 'Non of your business',
            },
        ).as('getOneStock');

        cy.mount(mount);

        cy.wait('@listStocks');
        cy.wait('@getOneStock');

        cy.get('[data-cy="stockBlueprintFilter"]').should('be.visible');
        cy.get('[data-cy="stockBlueprintActionGroup"]').should('be.visible');
        cy.get('[data-cy="stockBlueprintDataTable"]').should('be.visible');

        cy.get('[data-cy="structureLoadingError"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntries"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntriesAddButton"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintLoaderInitial"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderApi"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintEmptyNoSearchResult"]').should(
            'not.exist',
        );
    });

    it('should filter and show one item', () => {
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints',
            },
            ['6e63d58f-f291-435a-bc35-82c8b47d5527'],
        ).as('listStocks');
        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints/6e63d58f-f291-435a-bc35-82c8b47d5527',
            },
            {
                name: 'Test BPC stock',
                security: 'Non of your business',
            },
        ).as('getOneStock');

        cy.mount(mount);

        cy.wait('@listStocks');
        cy.wait('@getOneStock');

        cy.intercept(
            {
                method: 'GET',
                url: '/api/v1/stocks/blueprints?name=Test',
            },
            ['6e63d58f-f291-435a-bc35-82c8b47d5527'],
        ).as('filterActive');

        cy.get('[data-cy="filter-text"]').type('Test').type('{enter}');

        cy.wait('@filterActive');
        cy.wait('@getOneStock');

        cy.get('[data-cy="stockBlueprintFilter"]').should('be.visible');
        cy.get('[data-cy="stockBlueprintActionGroup"]').should('be.visible');
        cy.get('[data-cy="stockBlueprintDataTable"]').should('be.visible');

        cy.get('[data-cy="stockBlueprintLoadingError"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntries"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintNoEntriesAddButton"]').should(
            'not.exist',
        );
        cy.get('[data-cy="stockBlueprintLoaderInitial"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintLoaderApi"]').should('not.exist');
        cy.get('[data-cy="stockBlueprintEmptyNoSearchResult"]').should(
            'not.exist',
        );
    });
});
