<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_cvf-input-code-container     flex 1  Ve_7df075</name>
   <tag></tag>
   <elementGuidId>451436a9-16c5-42a7-b6a5-4bd59edea890</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#a-page</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='a-page']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>00dbacbc-0720-4163-b1ac-6e91e0a704b5</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>a-page</value>
      <webElementGuid>61ed1547-1f28-4824-8592-d209502b1a48</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Verification required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;33100e57-e8fc-4c46-b159-fd26f485d730&quot;}

            To continue, complete this verification step. 
            We've sent a One Time Password (OTP) to the email
 hariniguggilla.123@gmail.com. 
            Please enter it below. 



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, 'A message with a One Time Password (OTP) has been sent to', &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you've requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you've already received won't work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP


I need more helpIf you've already tried to reset your password, but haven't received an email from Amazon, check your Junk or Spam folder.If you can't access your email, try resetting that first through your email provider.If you've recently updated your password, your old password could still be saved in your browser. Try clearing your browser history and re-typing your password.If you need more password help, call us at 1-800-383-9484 or, if outside the U.S. or Canada, 1-206-577-1364 (International, charges may apply).

  P.when('A', 'ready').execute(function(A) {
    var $ = A.$;

    A.declarative('cvf-input-code-handler', 'keyup', function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $('.cvf-widget-link-resend').click(function() {
      $('.cvf-widget-form-resend').submit();
    });
  });







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



</value>
      <webElementGuid>3bf0d73d-5652-4db3-940f-7972c2a8fc67</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;a-page&quot;)</value>
      <webElementGuid>8689725d-67da-4fcc-832c-05eafde24a20</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='a-page']</value>
      <webElementGuid>0ba86f86-535f-4744-98a1-19dc601aa5c8</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div</value>
      <webElementGuid>5f4a2c01-5ab4-453b-bd6c-e0e8f148e256</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'a-page' and (text() = concat(&quot;{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Verification required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;33100e57-e8fc-4c46-b159-fd26f485d730&quot;}

            To continue, complete this verification step. 
            We&quot; , &quot;'&quot; , &quot;ve sent a One Time Password (OTP) to the email
 hariniguggilla.123@gmail.com. 
            Please enter it below. 



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, &quot; , &quot;'&quot; , &quot;A message with a One Time Password (OTP) has been sent to&quot; , &quot;'&quot; , &quot;, &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you&quot; , &quot;'&quot; , &quot;ve requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you&quot; , &quot;'&quot; , &quot;ve already received won&quot; , &quot;'&quot; , &quot;t work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP


I need more helpIf you&quot; , &quot;'&quot; , &quot;ve already tried to reset your password, but haven&quot; , &quot;'&quot; , &quot;t received an email from Amazon, check your Junk or Spam folder.If you can&quot; , &quot;'&quot; , &quot;t access your email, try resetting that first through your email provider.If you&quot; , &quot;'&quot; , &quot;ve recently updated your password, your old password could still be saved in your browser. Try clearing your browser history and re-typing your password.If you need more password help, call us at 1-800-383-9484 or, if outside the U.S. or Canada, 1-206-577-1364 (International, charges may apply).

  P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).execute(function(A) {
    var $ = A.$;

    A.declarative(&quot; , &quot;'&quot; , &quot;cvf-input-code-handler&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $(&quot; , &quot;'&quot; , &quot;.cvf-widget-link-resend&quot; , &quot;'&quot; , &quot;).click(function() {
      $(&quot; , &quot;'&quot; , &quot;.cvf-widget-form-resend&quot; , &quot;'&quot; , &quot;).submit();
    });
  });







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



&quot;) or . = concat(&quot;{}






  
    

    
      



  
  

  
  
    
    
      

    
      
    
  


    
  




  


  #cvf-input-code-container {
    flex: 1;
  }









Verification required

{&quot;isClientDrivenOtpSendingEnabled&quot;:false,&quot;arbParam&quot;:&quot;33100e57-e8fc-4c46-b159-fd26f485d730&quot;}

            To continue, complete this verification step. 
            We&quot; , &quot;'&quot; , &quot;ve sent a One Time Password (OTP) to the email
 hariniguggilla.123@gmail.com. 
            Please enter it below. 



Enter OTP        
        





Please check your code and try again.
Please enter the verification code.

  
    






  P.when(&quot;A&quot;, &quot;codeResendTimer&quot;, &quot;ready&quot;).execute(function(A, codeResendTimer) {
    A.$(document).ready(function() {
      var timer = codeResendTimer.createTimer(0, &quot; , &quot;'&quot; , &quot;A message with a One Time Password (OTP) has been sent to&quot; , &quot;'&quot; , &quot;, &quot;You can now request a new code if needed.&quot;, &quot;timer&quot;, &quot;timer&quot;, false);
    });
  });


Continue


        Resend OTP


      Resend OTP










Verification required
It looks like you&quot; , &quot;'&quot; , &quot;ve requested a new OTP recently. Please wait one minute before getting a new OTP. OTPs you&quot; , &quot;'&quot; , &quot;ve already received won&quot; , &quot;'&quot; , &quot;t work, so be sure to give the new OTP a moment to arrive.
You can now request a new code if needed.
Get new OTP


I need more helpIf you&quot; , &quot;'&quot; , &quot;ve already tried to reset your password, but haven&quot; , &quot;'&quot; , &quot;t received an email from Amazon, check your Junk or Spam folder.If you can&quot; , &quot;'&quot; , &quot;t access your email, try resetting that first through your email provider.If you&quot; , &quot;'&quot; , &quot;ve recently updated your password, your old password could still be saved in your browser. Try clearing your browser history and re-typing your password.If you need more password help, call us at 1-800-383-9484 or, if outside the U.S. or Canada, 1-206-577-1364 (International, charges may apply).

  P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ready&quot; , &quot;'&quot; , &quot;).execute(function(A) {
    var $ = A.$;

    A.declarative(&quot; , &quot;'&quot; , &quot;cvf-input-code-handler&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
      var VALID_DIGIT_REGEX = /^\d{6}$/;
      var $target = event.$target;

      if (VALID_DIGIT_REGEX.test($target.val())) {
        $(&quot;#verification-code-form&quot;).submit();
      }
    });

    $(&quot; , &quot;'&quot; , &quot;.cvf-widget-link-resend&quot; , &quot;'&quot; , &quot;).click(function() {
      $(&quot; , &quot;'&quot; , &quot;.cvf-widget-form-resend&quot; , &quot;'&quot; , &quot;).submit();
    });
  });







  .auth-footer-separator {
    display: inline-block;
    width: 20px;
  }





  
    
      
    

    
      
        
          
        

        
      

      
        

        
          
            Conditions of Use
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Privacy Notice
          
        
      

      
    
      
        
          
        

        
      

      
        

        
          
            Help
          
        
      

      
    
  

  
    
      © 1996-2024, Amazon.com, Inc. or its affiliates
    
  



&quot;))]</value>
      <webElementGuid>f6747893-65e3-4a69-af25-c55b5a4d47b4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
