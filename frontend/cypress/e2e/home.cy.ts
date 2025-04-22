describe("Home Page Testing", () => {
  it('Is Loaded', () => {
    cy.visit('/')
    cy.contains('h1', 'Traffic Flow Analysis')
  })
  
  it('VehicleCountChart is Rendered', () => {
    cy.visit('/')
    cy.get('#vehicle-count-chart').should('exist')
    cy.get('#vehicle-count-chart').should('be.visible')
  })
  
  it('TrafficFlowChart is Rendered', () => {
    cy.visit('/')
    cy.get('#traffic-flow-chart').should('exist')
    cy.get('#traffic-flow-chart').should('be.visible')
  })
  
  it('VehicleCountChart is Rendered', () => {
    cy.visit('/')
    cy.get('#weather-count-chart').should('exist')
    cy.get('#weather-count-chart').should('be.visible')
  })
})