import { MantineProvider } from "@mantine/core";
import { Filter, type FilterPropEntry } from "./Filter";

describe('Filter General', () => {
    it('should show two options and select the first one, and select the first one from that', () => {
        const exampleData: FilterPropEntry[] = [{
            key: 'a',
            label: 'Single Select',
            type: 'SELECT',
            options: [{
                key: 'aa',
                label: 'AA'
            }, {
                key: 'bb',
                label: 'BB'
            }, {
                key: 'cc',
                label: 'CC'
            }]
        }, {
            key: 'b',
            label: 'Another Select',
            type: 'SELECT',
            options: [{
                key: 'dd',
                label: 'DD'
            }]
        }];

        cy.mount(
            <MantineProvider>
                <Filter
                    entries={exampleData}
                    onFilterChange={() => {}}
                />
            </MantineProvider>
        );

        const firstSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        firstSelect.should('have.length', 2);
        firstSelect.first().click();
        cy
            .get('[data-cy="filterInput"]')
            .should('have.value', 'Single Select: ');

        const secondSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        secondSelect.should('have.length', 3);
        secondSelect.first().click();

        const selectedElements = cy.get('[data-cy="filterSelectedGroup"]');
        // span and input
        selectedElements.children().should('have.length', 2);
        selectedElements.first().should('have.text', 'Single Select: AA');
    });

    it('should show one multiselect and select all entries', () => {
        const exampleData: FilterPropEntry[] = [{
            key: 'a',
            label: 'Multiselect',
            type: 'MULTISELECT',
            options: [{
                key: 'aa',
                label: 'AA'
            }, {
                key: 'bb',
                label: 'BB'
            }, {
                key: 'cc',
                label: 'CC'
            }]
        }];

        cy.mount(
            <MantineProvider>
                <Filter
                    entries={exampleData}
                    onFilterChange={() => {}}
                />
            </MantineProvider>
        );

        const firstSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        firstSelect.should('have.length', 1);
        firstSelect.first().click();
        cy
            .get('[data-cy="filterInput"]')
            .should('have.value', 'Multiselect: ');

        const secondSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        secondSelect.should('have.length', 3);
        secondSelect.first().click();

        // select next option
        cy
            .get('[data-cy="filterSelectedGroup"]')
            .children()
            .last()
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children()
            .first()
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children()
            .first()
            .click();
        // select last option
        cy
            .get('[data-cy="filterSelectedGroup"]')
            .children()
            .last()
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children()
            .first()
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children()
            .first()
            .click();

        const selectedElements = cy.get('[data-cy="filterSelectedGroup"]').children();
        //// span, span, span and input
        selectedElements.should('have.length', 4);
        cy.get('[data-cy="filterSelectedGroup"]')
            .children()
            .each(($el, index) => {
                if (index === 0) {
                    cy.wrap($el).should('have.text', 'Multiselect: AA');
                } else if (index === 1) {
                    cy.wrap($el).should('have.text', 'Multiselect: BB');
                } else if (index === 2) {
                    cy.wrap($el).should('have.text', 'Multiselect: CC');
                }
            });
    });

    it('should show one input option', () => {
        const exampleData: FilterPropEntry[] = [{
            key: 'a',
            label: 'Input',
            type: 'STRING',
        }];

        cy.mount(
            <MantineProvider>
                <Filter
                    entries={exampleData}
                    onFilterChange={() => {}}
                />
            </MantineProvider>
        );

        const firstSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        firstSelect.should('have.length', 1);
        firstSelect.first().click();
        cy
            .get('[data-cy="filterInput"]')
            .should('have.value', 'Input: ');

        cy
            .get('[data-cy="filterInput"]')
            .click()
            .type('Test{enter}');

        const selectedElements = cy.get('[data-cy="filterSelectedGroup"]').children();
        //// span, and input
        selectedElements.should('have.length', 2);
        selectedElements.first().should('have.text', 'Input: Test')
    });

    it('should add an element and remove it', () => {
        const exampleData: FilterPropEntry[] = [{
            key: 'a',
            label: 'Single Select',
            type: 'SELECT',
            options: [{
                key: 'aa',
                label: 'AA'
            }, {
                key: 'bb',
                label: 'BB'
            }, {
                key: 'cc',
                label: 'CC'
            }]
        }];

        cy.mount(
            <MantineProvider>
                <Filter
                    entries={exampleData}
                    onFilterChange={() => {}}
                />
            </MantineProvider>
        );

        const firstSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        firstSelect.should('have.length', 1);
        firstSelect.first().click();
        cy
            .get('[data-cy="filterInput"]')
            .should('have.value', 'Single Select: ');

        const secondSelect = cy
            .get('[data-cy="filterCombobox"]')
            .click()
            .get('[data-cy="filterDropdownOption"]')
            .children();
        secondSelect.should('have.length', 3);
        secondSelect.first().click();

        const selectedElements = cy.get('[data-cy="filterSelectedGroup"]');
        // span and input
        selectedElements.children().should('have.length', 2);
        selectedElements.first().should('have.text', 'Single Select: AA');

        selectedElements.first().click();
        cy
            .get('[data-cy="filterSelectedGroup"]')
            .children()
            .should('have.length', 1);
    });
});
