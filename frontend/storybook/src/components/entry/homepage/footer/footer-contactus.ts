import "@elements/entry/home/TOSORT/column-details";
import "@elements/entry/home/TOSORT/column-list";

export default {
    title: 'Homepage Paragraph',
  }


  const STR_TITLE="Contact us";
  const STR_LINE1="info@jewishinteractive.org";
  const STR_LINE2="Ji United States";
  const STR_LINE3="Tel: +1 (703) 517-5182";
  const STR_LINE4="Ji United Kingdom";
  const STR_LINE5="Tel: +44 (0)79 6641 4417";
  const STR_LINE6="Ji South Africa";
  const STR_LINE7="Tel: +27 (79) 886 5326";
  const STR_LINE8="Ji Israel";
  const STR_LINE9="Tel: +972 (0) 54-597 9555"  ;
  const STR_LINE10="                         " ; 

  const STR_WHITE ="white";
  const STR_PATHYOUTUBE="Icn_Youtube.png";
  const STR_PATHFACEBOOK="Icn_Youtube.png";
  const STR_PATHINSTAGRAM="Icn_Youtube.png";
  const STR_PATHLINKEDIN="Icn_Youtube.png";



    export const footercontactus= () => {
    return `
    <column-details head_title="${STR_TITLE}">
    <column-list text_line="${STR_LINE1}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE2}" color="${STR_WHITE}" slot="list" bold=true ></column-list>
    <column-list text_line="${STR_LINE3}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE4}" color="${STR_WHITE}" bold=true slot="list"></column-list>
    <column-list text_line="${STR_LINE5}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE6}" color="${STR_WHITE}" bold=true slot="list"></column-list>
    <column-list text_line="${STR_LINE7}" color="${STR_WHITE}" slot="list"></column-list>
    <column-list text_line="${STR_LINE8}" color="${STR_WHITE}" slot="list" bold=true></column-list>
    <column-list text_line="${STR_LINE9}" color="${STR_WHITE}" slot="list" ></column-list>
   <div > </div>

    <social-networks slot="socialnetworks" path_instagram"${STR_PATHINSTAGRAM}" path_facebook="${STR_PATHFACEBOOK}"   path_youtube"${STR_PATHYOUTUBE}" path_linkedin"${STR_PATHLINKEDIN}"> </social-networks>

    

     </column-details>
    `
}