import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {modulePage, ModulePageKind} from "@components/module";
import {mockWords, mockThemes, nCardsToGrid} from "./common/mock-data";
import playerTmpl from "@templates/module/memory/play/player.html";
import cardPlayTmpl from "@templates/module/memory/play/memory-card.html";

export default {
  title: 'Modules/Memory-Game/Play'
}

const makePlayer = (nCards) => () =>  mockPlayer(0, false, nCards);
export const Play_8_Cards= makePlayer(8);
export const Play_10_Cards = makePlayer(10);
export const Play_12_Cards = makePlayer(12);
export const Play_14_Cards = makePlayer(14);
export const Play_16_Cards = makePlayer(16);
export const Play_18_Cards = makePlayer(18);
export const Play_20_Cards = makePlayer(20);
export const Play_22_Cards = makePlayer(22);
export const Play_24_Cards = makePlayer(24);
export const Play_26_Cards = makePlayer(26);
export const Play_28_Cards = makePlayer(28);

export const Player_Theme_1 = () => mockPlayer(1, false, 12);
export const Player_Theme_1_Flipped = () => mockPlayer(1, true, 12);
//Helpers

function mockPlayer(selectedThemeIndex, isFlipped, nCards) {
    const nGrid = nCardsToGrid(nCards);

    const main = tmpl(playerTmpl);

    appendMockCards(main, {isFlipped, nCards});
    if(selectedThemeIndex) {
        const {id} = mockThemes[selectedThemeIndex];
        toggleClasses(main, [`memory-theme-${id}`], true);
    }
    toggleClassesId(main, "game-cards", [`memory-grid-${nGrid}`], true);

    return modulePage({
        kind: ModulePageKind.Play,
        main
    })
}

function appendMockCards(main, {isFlipped, nCards}) {
    for(let i = 0; i < nCards; i++) {
        const word = mockWords[0];
        const card = tmpl(cardPlayTmpl);
        setTextId(card, "text-contents", word);
        appendId(main, "game-cards", card);
        toggleClassesId(card, "image", ["hidden"], true);
        if(isFlipped) {
            toggleClassesId(card, "flip", [`flip-card-clicked`], true);
        }
    }

    return main;
}