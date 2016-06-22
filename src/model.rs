// Copyright (c) 2016 Nikita Pekin and the wolfram_alpha_rs contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Struct and enum definitions of values in the Wolfram|Alpha model.
//!
//! For more information, see [Wolfram|Alpha's API
//! documentation](http://products.wolframalpha.com/api/documentation.html).

#![allow(missing_docs)]
#![cfg_attr(feature = "clippy", allow(cyclomatic_complexity))]

use url::Url;

#[cfg(feature = "nightly")]
include!("model.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/model.rs"));

#[cfg(test)]
mod tests {
    use serde_xml::from_str;

    use super::{Img, Infos, Pod, QueryResult, Subpod};

    #[test]
    fn test_query_result_deserializer() {
        let s = QUERY_RESULT_STR.to_owned();
        from_str::<QueryResult>(&s).unwrap();

        let s = QUERY_RESULT_2_STR.to_owned();
        from_str::<QueryResult>(&s).unwrap();
    }

    #[test]
    fn test_pod_deserializer() {
        let s = POD_STR.to_owned();
        from_str::<Pod>(&s).unwrap();
    }

    #[test]
    fn test_subpod_deserializer() {
        let s = SUBPOD_STR.to_owned();
        from_str::<Subpod>(&s).unwrap();
    }

    #[test]
    fn test_img_deserializer() {
        let s = IMG_STR.to_owned();
        from_str::<Img>(&s).unwrap();
    }

    #[test]
    fn test_infos_deserializer() {
        let s = INFOS_STR.to_owned();
        from_str::<Infos>(&s).unwrap();
    }

    const IMG_STR: &'static str = r#"<img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25401i8g731fgf05hh6h000017hf04b39af18gah?MSPStoreType=image/gif&s=18" alt="3.141592653589793238462643383279502884197169399375105820974..." title="3.141592653589793238462643383279502884197169399375105820974..." width="493" height="22"/>"#;

    const SUBPOD_STR: &'static str = r#"
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25391i8g731fgf05hh6h00004fe2i2b23b0edcbg?MSPStoreType=image/gif&amp;s=18" alt="pi" title="pi" width="9" height="18"/>
   <plaintext>pi</plaintext>
  </subpod>
    "#;

    const POD_STR: &'static str = r#"
 <pod title="Input" scanner="Identity" id="Input" position="100" error="false" numsubpods="1">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25391i8g731fgf05hh6h00004fe2i2b23b0edcbg?MSPStoreType=image/gif&amp;s=18" alt="pi" title="pi" width="9" height="18"/>
   <plaintext>pi</plaintext>
  </subpod>
 </pod>
    "#;

    const QUERY_RESULT_STR: &'static str = r#"
<queryresult success="true" error="false" numpods="8" datatypes="MathematicalFunctionIdentity" timedout="Numeric" timedoutpods="" timing="3.093" parsetiming="0.136" parsetimedout="false" recalculate="http://www3.wolframalpha.com/api/v2/recalc.jsp?id=MSPa25361i8g731fgf05hh6h0000264dcffc60f64ecd&amp;s=18" id="MSPa25371i8g731fgf05hh6h00002b67bbe0hh40gh78" host="http://www3.wolframalpha.com" server="18" related="http://www3.wolframalpha.com/api/v2/relatedQueries.jsp?id=MSPa25381i8g731fgf05hh6h00002hdb0abg74617081&amp;s=18" version="2.6">
 <pod title="Input" scanner="Identity" id="Input" position="100" error="false" numsubpods="1">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25391i8g731fgf05hh6h00004fe2i2b23b0edcbg?MSPStoreType=image/gif&amp;s=18" alt="pi" title="pi" width="9" height="18"/>
   <plaintext>pi</plaintext>
  </subpod>
 </pod>
 <pod title="Decimal approximation" scanner="Numeric" id="DecimalApproximation" position="200" error="false" numsubpods="1" primary="true">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25401i8g731fgf05hh6h000017hf04b39af18gah?MSPStoreType=image/gif&amp;s=18" alt="3.141592653589793238462643383279502884197169399375105820974..." title="3.141592653589793238462643383279502884197169399375105820974..." width="493" height="22"/>
   <plaintext>3.141592653589793238462643383279502884197169399375105820974...</plaintext>
  </subpod>
  <states count="1">
   <state name="More digits" input="DecimalApproximation__More digits"/>
  </states>
 </pod>
 <pod title="Property" scanner="Numeric" id="Property" position="300" error="false" numsubpods="1">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25411i8g731fgf05hh6h0000266a5814i2edf4bi?MSPStoreType=image/gif&amp;s=18" alt="pi is a transcendental number" title="pi is a transcendental number" width="194" height="18"/>
   <plaintext>pi is a transcendental number</plaintext>
  </subpod>
 </pod>
 <pod title="Number line" scanner="NumberLine" id="NumberLine" position="400" error="false" numsubpods="1">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25421i8g731fgf05hh6h00001ddecebc4fhchf53?MSPStoreType=image/gif&amp;s=18" alt="" title="" width="300" height="56"/>
   <plaintext/>
  </subpod>
 </pod>
 <pod title="Continued fraction" scanner="ContinuedFraction" id="ContinuedFraction" position="500" error="false" numsubpods="1">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25431i8g731fgf05hh6h00000edb9ce0cig3a457?MSPStoreType=image/gif&amp;s=18" alt="[3; 7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 2, 1, 1, 2, 2, 2, 2, 1, 84, 2, 1, 1, 15, 3, 13, ...]" title="[3; 7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 2, 1, 1, 2, 2, 2, 2, 1, 84, 2, 1, 1, 15, 3, 13, ...]" width="548" height="20"/>
   <plaintext>[3; 7, 15, 1, 292, 1, 1, 1, 2, 1, 3, 1, 14, 2, 1, 1, 2, 2, 2, 2, 1, 84, 2, 1, 1, 15, 3, 13, ...]</plaintext>
  </subpod>
  <states count="2">
   <state name="More terms" input="ContinuedFraction__More terms"/>
   <state name="Fraction form" input="ContinuedFraction__Fraction form"/>
  </states>
 </pod>
 <pod title="Alternative representations" scanner="MathematicalFunctionData" id="AlternativeRepresentations:MathematicalFunctionIdentityData" position="600" error="false" numsubpods="3">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25441i8g731fgf05hh6h000063h14f8e07g6a815?MSPStoreType=image/gif&amp;s=18" alt="pi = 180 °" title="pi = 180 °" width="61" height="28"/>
   <plaintext>pi = 180 °</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25451i8g731fgf05hh6h00005hgb2059b9dha9a0?MSPStoreType=image/gif&amp;s=18" alt="pi = -i log(-1)" title="pi = -i log(-1)" width="97" height="28"/>
   <plaintext>pi = -i log(-1)</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25461i8g731fgf05hh6h00005iec693fgdf0ff64?MSPStoreType=image/gif&amp;s=18" alt="pi = cos^(-1)(-1)" title="pi = cos^(-1)(-1)" width="94" height="28"/>
   <plaintext>pi = cos^(-1)(-1)</plaintext>
  </subpod>
  <states count="1">
   <state name="More" input="AlternativeRepresentations:MathematicalFunctionIdentityData__More"/>
  </states>
 </pod>
 <pod title="Series representations" scanner="MathematicalFunctionData" id="SeriesRepresentations:MathematicalFunctionIdentityData" position="700" error="false" numsubpods="3">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25501i8g731fgf05hh6h000031g7i936i3d94038?MSPStoreType=image/gif&amp;s=18" alt="pi = 4 sum_(k=0)^infinity (-1)^k/(2 k+1)" title="pi = 4 sum_(k=0)^infinity (-1)^k/(2 k+1)" width="110" height="56"/>
   <plaintext>pi = 4 sum_(k=0)^infinity (-1)^k/(2 k+1)</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25511i8g731fgf05hh6h00003b0552957d513383?MSPStoreType=image/gif&amp;s=18" alt="pi = -2+2 sum_(k=1)^infinity 2^k/(binomial(2 k, k))" title="pi = -2+2 sum_(k=1)^infinity 2^k/(binomial(2 k, k))" width="139" height="69"/>
   <plaintext>pi = -2+2 sum_(k=1)^infinity 2^k/(binomial(2 k, k))</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25521i8g731fgf05hh6h00002bc99dah1e761g30?MSPStoreType=image/gif&amp;s=18" alt="pi = sum_(k=0)^infinity (50 k-6)/(2^k binomial(3 k, k))" title="pi = sum_(k=0)^infinity (50 k-6)/(2^k binomial(3 k, k))" width="111" height="67"/>
   <plaintext>pi = sum_(k=0)^infinity (50 k-6)/(2^k binomial(3 k, k))</plaintext>
  </subpod>
  <states count="1">
   <state name="More" input="SeriesRepresentations:MathematicalFunctionIdentityData__More"/>
  </states>
  <infos count="2">
   <info text="(n m) is the binomial coefficient">
    <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25531i8g731fgf05hh6h000016f21e0b2i2498d1?MSPStoreType=image/gif&amp;s=18" alt="(n m) is the binomial coefficient" title="(n m) is the binomial coefficient" width="204" height="36"/>
    <link url="http://reference.wolfram.com/mathematica/ref/Binomial.html" text="Documentation" title="Mathematica"/>
    <link url="http://functions.wolfram.com/GammaBetaErf/Binomial" text="Properties" title="Wolfram Functions Site"/>
    <link url="http://mathworld.wolfram.com/BinomialCoefficient.html" text="Definition" title="MathWorld"/>
   </info>
   <info>
    <link url="http://functions.wolfram.com/Constants/Pi/06/ShowAll.html" text="More information"/>
   </info>
  </infos>
 </pod>
 <pod title="Integral representations" scanner="MathematicalFunctionData" id="IntegralRepresentations:MathematicalFunctionIdentityData" position="800" error="false" numsubpods="3">
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25541i8g731fgf05hh6h00003b0h458cb25607ib?MSPStoreType=image/gif&amp;s=18" alt="pi = 2 integral_0^infinity 1/(t^2+1) dt" title="pi = 2 integral_0^infinity 1/(t^2+1) dt" width="126" height="44"/>
   <plaintext>pi = 2 integral_0^infinity 1/(t^2+1) dt</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25551i8g731fgf05hh6h00004agf247db1a7be0e?MSPStoreType=image/gif&amp;s=18" alt="pi = 4 integral_0^1 sqrt(1-t^2) dt" title="pi = 4 integral_0^1 sqrt(1-t^2) dt" width="138" height="45"/>
   <plaintext>pi = 4 integral_0^1 sqrt(1-t^2) dt</plaintext>
  </subpod>
  <subpod title="">
   <img src="http://www3.wolframalpha.com/Calculate/MSP/MSP25561i8g731fgf05hh6h000040a90028eedi4eh0?MSPStoreType=image/gif&amp;s=18" alt="pi = 2 integral_0^infinity (sin(t))/t dt" title="pi = 2 integral_0^infinity (sin(t))/t dt" width="124" height="45"/>
   <plaintext>pi = 2 integral_0^infinity (sin(t))/t dt</plaintext>
  </subpod>
  <states count="1">
   <state name="More" input="IntegralRepresentations:MathematicalFunctionIdentityData__More"/>
  </states>
  <infos count="1">
   <info>
    <link url="http://functions.wolfram.com/Constants/Pi/07/ShowAll.html" text="More information"/>
   </info>
  </infos>
 </pod>
 <assumptions count="1">
  <assumption type="Clash" word="pi" template="Assuming &quot;${word}&quot; is ${desc1}. Use as ${desc2} instead" count="6">
   <value name="NamedConstant" desc="a mathematical constant" input="*C.pi-_*NamedConstant-"/>
   <value name="Character" desc="a character" input="*C.pi-_*Character-"/>
   <value name="MathWorld" desc=" referring to a mathematical definition" input="*C.pi-_*MathWorld-"/>
   <value name="MathWorldClass" desc="a class of mathematical terms" input="*C.pi-_*MathWorldClass-"/>
   <value name="Movie" desc="a movie" input="*C.pi-_*Movie-"/>
   <value name="Word" desc="a word" input="*C.pi-_*Word-"/>
  </assumption>
 </assumptions>
</queryresult>
    "#;

    const QUERY_RESULT_2_STR: &'static str = r#"
    <?xml version='1.0' encoding='UTF-8'?>
<queryresult success='true'
    error='false'
    numpods='6'
    datatypes='BankRate,Formula'
    timedout=''
    timedoutpods=''
    timing='4.893'
    parsetiming='0.211'
    parsetimedout='false'
    recalculate=''
    id='MSPa4021i7idga8575i19e5000028ah45b9fc3dbi1h'
    host='http://www4b.wolframalpha.com'
    server='55'
    related='http://www4b.wolframalpha.com/api/v2/relatedQueries.jsp?id=MSPa4031i7idga8575i19e5000011d974b39idc35a56033799198928217484'
    version='2.6'>
 <pod title='Input interpretation'
     scanner='Formula'
     id='Input'
     position='100'
     error='false'
     numsubpods='1'>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4041i7idga8575i19e500000g0eg8g637d61ce3?MSPStoreType=image/gif&amp;s=55'
       alt='fixed rate mortgage'
       title='fixed rate mortgage'
       width='145'
       height='23' />
   <plaintext>fixed rate mortgage</plaintext>
  </subpod>
 </pod>
 <pod title='Input values'
     scanner='Formula'
     id='InputValue'
     position='200'
     error='false'
     numsubpods='1'>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4051i7idga8575i19e500000cce954f8574d1dg?MSPStoreType=image/gif&amp;s=55'
       alt='loan amount | C$200000  (Canadian dollars)
loan period | 30 years
annual percentage rate | 30-year fixed-rate mortgage: 3.55%'
       title='loan amount | C$200000  (Canadian dollars)
loan period | 30 years
annual percentage rate | 30-year fixed-rate mortgage: 3.55%'
       width='444'
       height='100' />
   <plaintext>loan amount | C$200000  (Canadian dollars)
loan period | 30 years
annual percentage rate | 30-year fixed-rate mortgage: 3.55%</plaintext>
  </subpod>
 </pod>
 <pod title='Monthly payments'
     scanner='Formula'
     id='MonthlyPayments'
     position='300'
     error='false'
     numsubpods='1'
     primary='true'>
  <subpod title=''
      primary='true'>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4061i7idga8575i19e500003g189d5661cid859?MSPStoreType=image/gif&amp;s=55'
       alt='monthly payment | C$904
effective interest rate | 3.608%'
       title='monthly payment | C$904
effective interest rate | 3.608%'
       width='232'
       height='68' />
   <plaintext>monthly payment | C$904
effective interest rate | 3.608%</plaintext>
  </subpod>
  <infos count='1'>
   <info>
    <units count='1'>
     <unit short='C$1'
         long='Canadian dollars' />
     <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4071i7idga8575i19e500000i1d6e37306515ge?MSPStoreType=image/gif&amp;s=55'
         width='158'
         height='26' />
    </units>
   </info>
  </infos>
 </pod>
 <pod title='Mortgage totals'
     scanner='Formula'
     id='MortgageTotals'
     position='400'
     error='false'
     numsubpods='1'>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4081i7idga8575i19e50000472i7g1fc67ec2ed?MSPStoreType=image/gif&amp;s=55'
       alt='principal paid | C$200000
total interest paid | C$125325
total payments | C$325325 | '
       title='principal paid | C$200000
total interest paid | C$125325
total payments | C$325325 | '
       width='493'
       height='103' />
   <plaintext>principal paid | C$200000
total interest paid | C$125325
total payments | C$325325 | </plaintext>
  </subpod>
  <infos count='1'>
   <info>
    <units count='1'>
     <unit short='C$1'
         long='Canadian dollars' />
     <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4091i7idga8575i19e500000g9iadief25hehad?MSPStoreType=image/gif&amp;s=55'
         width='158'
         height='26' />
    </units>
   </info>
  </infos>
 </pod>
 <pod title='Payments and balances'
     scanner='Formula'
     id='PaymentsAndBalances'
     position='500'
     error='false'
     numsubpods='2'>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4101i7idga8575i19e50000302046ch5811hefh?MSPStoreType=image/gif&amp;s=55'
       alt=''
       title=''
       width='378'
       height='139' />
   <plaintext></plaintext>
  </subpod>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4111i7idga8575i19e500005e7bi7e328d0c282?MSPStoreType=image/gif&amp;s=55'
       alt=''
       title=''
       width='320'
       height='135' />
   <plaintext></plaintext>
  </subpod>
 </pod>
 <pod title='Payments table'
     scanner='Formula'
     id='PaymentsTable'
     position='600'
     error='false'
     numsubpods='1'>
  <subpod title=''>
   <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4121i7idga8575i19e5000048280020i78gb9ff?MSPStoreType=image/gif&amp;s=55'
       alt='year | monthly payment | ending balance | yearly principal paid | yearly interest paid
1 | C$904 | C$196194 | C$3806 | C$7038
2 | C$904 | C$192251 | C$3943 | C$6901
3 | C$904 | C$188166 | C$4085 | C$6759
4 | C$904 | C$183933 | C$4233 | C$6611
5 | C$904 | C$179548 | C$4385 | C$6459
6 | C$904 | C$175004 | C$4544 | C$6300
7 | C$904 | C$170297 | C$4708 | C$6137
8 | C$904 | C$165419 | C$4877 | C$5967
9 | C$904 | C$160366 | C$5053 | C$5791
10 | C$904 | C$155130 | C$5236 | C$5608
11 | C$904 | C$149705 | C$5425 | C$5419
12 | C$904 | C$144084 | C$5621 | C$5224
13 | C$904 | C$138261 | C$5823 | C$5021
14 | C$904 | C$132228 | C$6033 | C$4811
15 | C$904 | C$125977 | C$6251 | C$4593
16 | C$904 | C$119500 | C$6477 | C$4367
17 | C$904 | C$112789 | C$6710 | C$4134
18 | C$904 | C$105837 | C$6953 | C$3892
19 | C$904 | C$98633 | C$7203 | C$3641
20 | C$904 | C$91170 | C$7463 | C$3381
21 | C$904 | C$83437 | C$7733 | C$3112
22 | C$904 | C$75426 | C$8012 | C$2833
23 | C$904 | C$67125 | C$8301 | C$2543
24 | C$904 | C$58525 | C$8600 | C$2244
25 | C$904 | C$49614 | C$8911 | C$1934
26 | C$904 | C$40382 | C$9232 | C$1612
27 | C$904 | C$30817 | C$9565 | C$1279
28 | C$904 | C$20906 | C$9910 | C$934
29 | C$904 | C$10638 | C$10268 | C$576
30 | C$904 | C$0 | C$10638 | C$206'
       title='year | monthly payment | ending balance | yearly principal paid | yearly interest paid
1 | C$904 | C$196194 | C$3806 | C$7038
2 | C$904 | C$192251 | C$3943 | C$6901
3 | C$904 | C$188166 | C$4085 | C$6759
4 | C$904 | C$183933 | C$4233 | C$6611
5 | C$904 | C$179548 | C$4385 | C$6459
6 | C$904 | C$175004 | C$4544 | C$6300
7 | C$904 | C$170297 | C$4708 | C$6137
8 | C$904 | C$165419 | C$4877 | C$5967
9 | C$904 | C$160366 | C$5053 | C$5791
10 | C$904 | C$155130 | C$5236 | C$5608
11 | C$904 | C$149705 | C$5425 | C$5419
12 | C$904 | C$144084 | C$5621 | C$5224
13 | C$904 | C$138261 | C$5823 | C$5021
14 | C$904 | C$132228 | C$6033 | C$4811
15 | C$904 | C$125977 | C$6251 | C$4593
16 | C$904 | C$119500 | C$6477 | C$4367
17 | C$904 | C$112789 | C$6710 | C$4134
18 | C$904 | C$105837 | C$6953 | C$3892
19 | C$904 | C$98633 | C$7203 | C$3641
20 | C$904 | C$91170 | C$7463 | C$3381
21 | C$904 | C$83437 | C$7733 | C$3112
22 | C$904 | C$75426 | C$8012 | C$2833
23 | C$904 | C$67125 | C$8301 | C$2543
24 | C$904 | C$58525 | C$8600 | C$2244
25 | C$904 | C$49614 | C$8911 | C$1934
26 | C$904 | C$40382 | C$9232 | C$1612
27 | C$904 | C$30817 | C$9565 | C$1279
28 | C$904 | C$20906 | C$9910 | C$934
29 | C$904 | C$10638 | C$10268 | C$576
30 | C$904 | C$0 | C$10638 | C$206'
       width='546'
       height='1013' />
   <plaintext>year | monthly payment | ending balance | yearly principal paid | yearly interest paid
1 | C$904 | C$196194 | C$3806 | C$7038
2 | C$904 | C$192251 | C$3943 | C$6901
3 | C$904 | C$188166 | C$4085 | C$6759
4 | C$904 | C$183933 | C$4233 | C$6611
5 | C$904 | C$179548 | C$4385 | C$6459
6 | C$904 | C$175004 | C$4544 | C$6300
7 | C$904 | C$170297 | C$4708 | C$6137
8 | C$904 | C$165419 | C$4877 | C$5967
9 | C$904 | C$160366 | C$5053 | C$5791
10 | C$904 | C$155130 | C$5236 | C$5608
11 | C$904 | C$149705 | C$5425 | C$5419
12 | C$904 | C$144084 | C$5621 | C$5224
13 | C$904 | C$138261 | C$5823 | C$5021
14 | C$904 | C$132228 | C$6033 | C$4811
15 | C$904 | C$125977 | C$6251 | C$4593
16 | C$904 | C$119500 | C$6477 | C$4367
17 | C$904 | C$112789 | C$6710 | C$4134
18 | C$904 | C$105837 | C$6953 | C$3892
19 | C$904 | C$98633 | C$7203 | C$3641
20 | C$904 | C$91170 | C$7463 | C$3381
21 | C$904 | C$83437 | C$7733 | C$3112
22 | C$904 | C$75426 | C$8012 | C$2833
23 | C$904 | C$67125 | C$8301 | C$2543
24 | C$904 | C$58525 | C$8600 | C$2244
25 | C$904 | C$49614 | C$8911 | C$1934
26 | C$904 | C$40382 | C$9232 | C$1612
27 | C$904 | C$30817 | C$9565 | C$1279
28 | C$904 | C$20906 | C$9910 | C$934
29 | C$904 | C$10638 | C$10268 | C$576
30 | C$904 | C$0 | C$10638 | C$206</plaintext>
  </subpod>
  <infos count='1'>
   <info>
    <units count='1'>
     <unit short='C$1'
         long='Canadian dollars' />
     <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4131i7idga8575i19e50000601bfiibd3494b88?MSPStoreType=image/gif&amp;s=55'
         width='158'
         height='26' />
    </units>
   </info>
  </infos>
 </pod>
 <assumptions count='7'>
  <assumption type='Clash'
      word='mortgage'
      template='Assuming &quot;${word}&quot; is ${desc1}. Use as ${desc2} instead'
      count='3'>
   <value name='Formula'
       desc='a formula'
       input='*C.mortgage-_*Formula-' />
   <value name='MathWorld'
       desc=' referring to a mathematical definition'
       input='*C.mortgage-_*MathWorld-' />
   <value name='Word'
       desc='a word'
       input='*C.mortgage-_*Word-' />
  </assumption>
  <assumption type='FormulaSelect'
      template='Assuming ${desc1}. Use ${desc2} instead'
      count='2'>
   <value name='FixedRateMortgage'
       desc='fixed rate mortgage'
       input='FSelect_**FixedRateMortgage--' />
   <value name='AdjustableRateMortgage'
       desc='adjustable rate mortgage'
       input='FSelect_**AdjustableRateMortgage--' />
  </assumption>
  <assumption type='FormulaVariable'
      desc='loan amount'
      current='1'
      count='1'>
   <value name='FixedRateMortgage.MA'
       desc='C$200000'
       valid='true'
       input='*F.FixedRateMortgage.MA-_C%24200000' />
  </assumption>
  <assumption type='FormulaVariable'
      desc='loan period'
      current='1'
      count='1'>
   <value name='FixedRateMortgage.MP'
       desc='30 yr'
       valid='true'
       input='*F.FixedRateMortgage.MP-_30+yr' />
  </assumption>
  <assumption type='FormulaVariable'
      desc='annual percentage rate'
      current='1'
      count='1'>
   <value name='FixedRateMortgage.APR'
       desc='3.55 %'
       valid='true'
       input='*F.FixedRateMortgage.APR-_3.55+%25' />
  </assumption>
  <assumption type='FormulaVariableOption'
      template='Assuming ${desc1}. Use ${desc2} instead'
      count='2'>
   <value name='FixedRateMortgage.MA'
       desc='loan amount'
       input='*FVarOpt-_**FixedRateMortgage.MA--' />
   <value name='FixedRateMortgage.SA,FixedRateMortgage.DP'
       desc='sale amount and down payment'
       input='*FVarOpt-_**FixedRateMortgage.SA-.*FixedRateMortgage.DP--' />
  </assumption>
  <assumption type='FormulaVariableInclude'
      template='Also include ${desc1}'
      count='4'>
   <value name='FixedRateMortgage.PTS'
       desc='points'
       input='*FVarOpt-_**FixedRateMortgage.PTS-.*FixedRateMortgage.MA--' />
   <value name='FixedRateMortgage.IOP'
       desc='interest‐only period'
       input='*FVarOpt-_**FixedRateMortgage.IOP-.*FixedRateMortgage.MA--' />
   <value name='FixedRateMortgage.MTR'
       desc='tax rate'
       input='*FVarOpt-_**FixedRateMortgage.MTR-.*FixedRateMortgage.MA--' />
   <value name='FixedRateMortgage.BP'
       desc='balloon payment'
       input='*FVarOpt-_**FixedRateMortgage.BP-.*FixedRateMortgage.MA--' />
  </assumption>
 </assumptions>
 <sources count='1'>
  <source url='http://www.wolframalpha.com/sources/BankRateDataSourceInformationNotes.html'
      text='Bank rate data' />
 </sources>
</queryresult>
    "#;

    const INFOS_STR: &'static str = r#"
  <infos count='1'>
   <info>
    <units count='1'>
     <unit short='C$1'
         long='Canadian dollars' />
     <img src='http://www4b.wolframalpha.com/Calculate/MSP/MSP4071i7idga8575i19e500000i1d6e37306515ge?MSPStoreType=image/gif&amp;s=55'
         width='158'
         height='26' />
    </units>
   </info>
  </infos>
    "#;
}
