describe("Explore Page Testing", () => {
  it('Is Loaded', () => {
    cy.visit('/explores')
    cy.contains('h1', 'Play around with our APIs')
  })
  
  it('Verify that the selectable times are correct', () => {
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction').get('#time-selection').find('option').should('have.length', 49)
      .should('contain', 'Now')
      .should('contain', '00:00')
      .should('contain', '23:30')
    
  })
  
  it('Verify that the Time shown on the card changes when the user chooses it from the combobox', () => {
    // Set clock
    const currentTime = new Date(2025, 0, 1, 1, 1, 0);
    cy.clock(currentTime.getTime());
    
    cy.visit('/explores')

    cy.get('#vehicle-count-prediction #time-card').should('contain', '01:30');
    
    cy.get('#vehicle-count-prediction #time-selection').select('00:00');
    cy.get('#vehicle-count-prediction #time-card').should('contain', '00:00');
  })
  
  it('Verify that the selectable weather conditions are correct', () => {
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #weather-selection').find('option').should('have.length', 4)
      .should('contain', 'Clear')
      .should('contain', 'Cloud')
      .should('contain', 'Rain')
      .should('contain', 'Low Visibility')
  })
  
  it('Verify that the Weather shown on the card changes when the user chooses it from the combobox', () => {
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #weather-card svg path').invoke('attr', 'd').then((initial) => {
      cy.get('#vehicle-count-prediction #weather-card').should('contain', 'Clear');
      cy.get('#vehicle-count-prediction #weather-selection').select('Rain');
      cy.get('#vehicle-count-prediction #weather-card').should('contain', 'Rain');
      cy.get('#vehicle-count-prediction #weather-card svg path').invoke('attr', 'd').should('not.equal', initial);
    })
  });
  
  it('Verify that the selectable vehicle types are correct', () => {
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #vehicle-selection').find('option').should('have.length', 4)
      .should('contain', 'All')
      .should('contain', 'Motorcycle')
      .should('contain', 'Car')
      .should('contain', 'HeavyVehicle')
  })
  
  it('Verify that the vehicle type shown on the card changes when the user chooses it from the combobox', () => {
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #vehicle-card').find('svg').should('have.length', 3);
    cy.get('#vehicle-count-prediction #vehicle-selection').select('Motorcycle');
    cy.get('#vehicle-count-prediction #vehicle-card').find('svg').should('have.length', 1);
  });
  
  it('Verify that the API endpoint is properly configured when the user submits the exploration form', () => {
    const currentTime = new Date(2025, 0, 1, 1, 1, 0);
    cy.clock(currentTime.getTime());
    
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #vehicle-selection').select('Car');
    cy.get('#vehicle-count-prediction #time-selection').select('00:00');
    cy.get('#vehicle-count-prediction #weather-selection').select('Clear');
    
    cy.intercept('*').as('postRequest')
    cy.get('#vehicle-count-prediction button').click()
    cy.get('#vehicle-count-prediction button').should('be.disabled');
    cy.wait('@postRequest').then((interception) => {
      expect(interception.request.method).equal('POST');
      expect(interception.request.url).contain('/count/car');
      expect(JSON.stringify(interception.request.body)).equal(JSON.stringify({ time: "2025-01-01T00:00:00+07:00", weather_cond: 'Clear' }));
    });
  })
  
  it('Verify that predicted vehicle count is displayed', () => {
    const currentTime = new Date(2025, 0, 1, 1, 1, 0);
    cy.clock(currentTime.getTime());
    
    cy.visit('/explores')
    cy.get('#vehicle-count-prediction #vehicle-selection').select('Car');
    cy.get('#vehicle-count-prediction #time-selection').select('00:00');
    cy.get('#vehicle-count-prediction #weather-selection').select('Clear');
    
    cy.intercept('POST', '*', {
      body: {
        count: 10.23,
        vehicle_type: 'Car'
      }
    }).as('postRequest')
    cy.get('#vehicle-count-prediction button').click();
    cy.wait('@postRequest')
    cy.get('#vehicle-count-prediction').contains('Predicted Vehicle Count');
  })
})
