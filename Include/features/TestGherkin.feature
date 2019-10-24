@tag
Feature: Validar petición

  @tag1
  Scenario: Enviar request
    Given Que se envía la solicitud de saldos y bonos
    When La petición sea válida
    Then el estado de la respuesta HTTP ok
  
  @tag2
  Scenario: Validar campos del response
    Given Que se envía la solicitud de saldos y bonos en ok
    When Encuentre la respuesta del servicio 0000
    Then el codigo de ejecución es correcto
  