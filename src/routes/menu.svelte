<script>
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
            let menuItems = [home, voyage, daily];
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

    let home = new MenuItem('Home', '/');
    let voyage = new MenuItem('Voyage Report', '/voyage-report');
    let daily = new MenuItem('Daily Report', '/daily-report');
</script>

<ul class="menu">
    <li><a {...home} class={[home.getClass()]}>{home.title}</a></li>
    <li><a {...voyage} class={[voyage.getClass()]}>{voyage.title}</a></li>
    <li><a {...daily} class={[daily.getClass()]}>{daily.title}</a></li>
</ul>

<style>
.menu {
    list-style-type: none;
    padding: 0 0.5em;
}

.menu-item {
    display: block;
    box-sizing: border-box;
    width: 100%;
    line-height: 2.5em;
    text-indent: 1em;
    cursor: pointer;
}

.menu-item.selected {
    border-radius: 0.5em;
    background-color: red;
}

.menu-item:hover {
    font-weight: bold;
}
</style>
