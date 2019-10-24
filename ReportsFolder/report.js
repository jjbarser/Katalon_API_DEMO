$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("Include/features/TestGherkin.feature");
formatter.feature({
  "name": "Validar petición",
  "description": "",
  "keyword": "Feature",
  "tags": [
    {
      "name": "@tag"
    }
  ]
});
formatter.scenario({
  "name": "Enviar request",
  "description": "",
  "keyword": "Scenario",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag1"
    }
  ]
});
formatter.step({
  "name": "Que se envía la solicitud de saldos y bonos",
  "keyword": "Given "
});
formatter.match({
  "location": "RequestValidationStep.sendRequest()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "La petición sea válida",
  "keyword": "When "
});
formatter.match({
  "location": "RequestValidationStep.requestValidate()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "el estado de la respuesta HTTP ok",
  "keyword": "Then "
});
formatter.match({
  "location": "RequestValidationStep.responseStatus()"
});
formatter.result({
  "status": "passed"
});
formatter.scenario({
  "name": "Validar campos del response",
  "description": "",
  "keyword": "Scenario",
  "tags": [
    {
      "name": "@tag"
    },
    {
      "name": "@tag2"
    }
  ]
});
formatter.step({
  "name": "Que se envía la solicitud de saldos y bonos en ok",
  "keyword": "Given "
});
formatter.match({
  "location": "TagCodeValidation.sendRequest()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Encuentre la respuesta del servicio 0000",
  "keyword": "When "
});
formatter.match({
  "location": "TagCodeValidation.validateRequest()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "el codigo de ejecución es correcto",
  "keyword": "Then "
});
formatter.match({
  "location": "TagCodeValidation.containCode()"
});
formatter.result({
  "status": "passed"
});
});