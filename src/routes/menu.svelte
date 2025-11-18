<script>
    import { onMount } from "svelte";

class MenuItem {
    selected = $state(false);
    /**
     * @param {string} title
     * @param {string} href
     */
    constructor(title, href) {
        this.title = title;
        this.href = href;
        this.onclick = this.select;
    }

    select() {
        // let menuItems = [home, voyage, daily, test];
        Object.values(menuItems).forEach(menuItem => {
            if (this.title != menuItem.title) {
                menuItem.selected = false;
            } else {
                menuItem.selected = true;
            }
        });
    }

    getClass() {
        return {
            "menu-item": true,
            selected: this.selected
        }
    }
}

const menuItems = [
    new MenuItem('Home', '/'),
    new MenuItem('Voyage Report', '/voyage-report'),
    new MenuItem('Daily Report', '/daily-report'),
    new MenuItem('Test', '/test-page')
];

onMount(() => {
    menuItems.at(0)?.select();
});
</script>

<ul class="menu">
    {#each menuItems as item}
        <li><a {...item} class={[item.getClass()]}>{item.title}</a></li>
    {/each}
</ul>

<style>
.menu {
    list-style-type: none;
    padding: 0 0.5em;
}

.menu-item {
    display: block;
    color: black;
    box-sizing: border-box;
    width: 100%;
    line-height: 2.5em;
    text-indent: 1em;
    cursor: pointer;
}

.menu-item.selected {
    border-radius: 0.5em;
    /* background-color: coral; */
    background-color: rgb(251, 181, 132);
    font-weight: bold;
}

.menu-item:hover {
    font-weight: bold;
}
</style>
