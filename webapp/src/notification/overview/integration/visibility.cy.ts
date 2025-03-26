import { h } from 'vue';
import { NNotificationProvider } from 'naive-ui';
import NotificationOverview from '@/notification/overview/View.vue'

const mount = <any>h(
    NNotificationProvider,
    {},
    [
        h(NotificationOverview)
    ]
);

describe('Visibility', () => {
    it('should show the spinner', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications'
                },
                req => {
                    // do nothing with the req, only call the response with a 10s delay.
                    req.continue(res => {
                    res.delay = 10000;
                    res.send();
                    });
                },
            )
            .as('getTimeout');

        cy.mount(
            mount, {
        });

        cy
            .get('[data-cy="loaderInitial"]')
            .should('be.visible');

        cy
            .get('[data-cy="loadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="filter"]')
            .should('not.exist');
        cy
            .get('[data-cy="actionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="noSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="dataTable"]')
            .should('not.exist');
    });

    it('should show no entries and have the option to create one', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications'
                },
                []
            )
            .as('getEmpty');

        cy.mount(
            mount, {
        });

        cy.wait('@getEmpty');

        cy
            .get('[data-cy="noEntries"]')
            .should('be.visible');
        cy
            .get('[data-cy="noEntriesAddButton"]')
            .should('be.visible');

        cy
            .get('[data-cy="loadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="filter"]')
            .should('not.exist');
        cy
            .get('[data-cy="actionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="noSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="dataTable"]')
            .should('not.exist');
    });

    it('should show the initial loading error message', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications'
                },
                {
                    statusCode: 500,
                }
            )
            .as('getError');

        cy.mount(
            mount, {
        });

        cy.wait('@getError');

        cy
            .get('[data-cy="loadingError"]')
            .should('be.visible');

        cy
            .get('[data-cy="noEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="filter"]')
            .should('not.exist');
        cy
            .get('[data-cy="actionGroup"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="noSearchResult"]')
            .should('not.exist');
        cy
            .get('[data-cy="dataTable"]')
            .should('not.exist');
    });

    it('should show one item', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications'
                },
                [
                    '6e63d58f-f291-435a-bc35-82c8b47d5527'
                ]
            )
            .as('list');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications/6e63d58f-f291-435a-bc35-82c8b47d5527'
                },
                {
                    name:   'Test Notification',
                    url:    'https://my.test.url',
                    target: 'JSON'
                }
            )
            .as('getOne');

        cy.mount(
            mount, {
        });

        cy.wait('@list');
        cy.wait('@getOne');

        cy
            .get('[data-cy="filter"]')
            .should('be.visible');
        cy
            .get('[data-cy="actionGroup"]')
            .should('be.visible');
        cy
            .get('[data-cy="dataTable"]')
            .should('be.visible');

        cy
            .get('[data-cy="loadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="noSearchResult"]')
            .should('not.exist');
    });

    it('should filter and show one item', () => {
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications'
                },
                [
                    '6e63d58f-f291-435a-bc35-82c8b47d5527'
                ]
            )
            .as('list');
        cy.intercept({
                    method: 'GET',
                    url: '/api/v1/notifications/6e63d58f-f291-435a-bc35-82c8b47d5527'
                },
                {
                    name:   'Test Notification',
                    url:    'https://my.test.url',
                    target: 'JSON'
                }
            )
            .as('getOne');

        cy.mount(
            mount, {
        });

        cy.wait('@list');
        cy.wait('@getOne');

        cy.intercept({
                method: 'GET',
                url: '/api/v1/notifications?name=Test'
            },
            [
                '6e63d58f-f291-435a-bc35-82c8b47d5527'
            ])
            .as('filterActive');

        cy.get('[data-cy="filter-text"]')
            .type('Test')
            .type('{enter}');

        cy.wait('@filterActive');
        cy.wait('@getOne');

        cy
            .get('[data-cy="filter"]')
            .should('be.visible');
        cy
            .get('[data-cy="actionGroup"]')
            .should('be.visible');
        cy
            .get('[data-cy="dataTable"]')
            .should('be.visible');

        cy
            .get('[data-cy="loadingError"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntries"]')
            .should('not.exist');
        cy
            .get('[data-cy="noEntriesAddButton"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderInitial"]')
            .should('not.exist');
        cy
            .get('[data-cy="loaderApi"]')
            .should('not.exist');
        cy
            .get('[data-cy="noSearchResult"]')
            .should('not.exist');
    });
});
