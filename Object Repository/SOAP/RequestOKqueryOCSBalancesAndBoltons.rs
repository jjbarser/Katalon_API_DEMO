<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RequestOKqueryOCSBalancesAndBoltons</name>
   <tag></tag>
   <elementGuidId>8498e691-77c5-43cd-8433-3ca02ac2af62</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v1=&quot;http://telefonica.ec/CommonBusinessEntities/BusinessInteractionABE/HeaderInfo/v1&quot; xmlns:typ=&quot;http://telefonica.ec/Customer/ChargeCalculationAndBalanceManagement/BalanceManagement/ManageBalanceOperations/v2/types&quot; xmlns:v11=&quot;http://telefonica.ec/Customer/CustomerABE/SubscriberIdentification/v1&quot;>
   &lt;soapenv:Header>
      &lt;v1:headerInfo>
         &lt;v1:application>WHATSAPP&lt;/v1:application>
         &lt;v1:serviceChannel>WHATSAPP&lt;/v1:serviceChannel>
         &lt;v1:password>WHATSAPP_022019&lt;/v1:password>
         &lt;v1:userLogin>XXX&lt;/v1:userLogin>
         &lt;v1:ipAddress>172.16.4.6&lt;/v1:ipAddress>
         &lt;v1:transactionTimestamp>2017-01-01T11:57:16&lt;/v1:transactionTimestamp>
         &lt;v1:serviceName>XXXX&lt;/v1:serviceName>
         &lt;v1:version>1.0&lt;/v1:version>
      &lt;/v1:headerInfo>
   &lt;/soapenv:Header>
   &lt;soapenv:Body>
      &lt;typ:queryOCSBalancesAndBoltonsRequest>
         &lt;typ:subscriberIdentification>
            &lt;v11:identificationValue>987421201&lt;/v11:identificationValue>
            &lt;v11:identificationType>MSISDN&lt;/v11:identificationType>
         &lt;/typ:subscriberIdentification>
         &lt;typ:subscriberType>VPN&lt;/typ:subscriberType>
         &lt;typ:tariffPlanId>1H&lt;/typ:tariffPlanId>
         &lt;!--Optional:-->
         &lt;typ:getTheSumOfBoltons>false&lt;/typ:getTheSumOfBoltons>
         &lt;!--Optional:-->
         &lt;typ:getBalances>true&lt;/typ:getBalances>
      &lt;/typ:queryOCSBalancesAndBoltonsRequest>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>queryOCSBalancesAndBoltons</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)











</verificationScript>
   <wsdlAddress>http://10.112.229.140:9011/Customer/ChargeCalculationAndBalanceManagement/BalanceManagement/ManageBalanceOperations/v3?wsdl</wsdlAddress>
</WebServiceRequestEntity>
