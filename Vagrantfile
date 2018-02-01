Vagrant.configure("2") do |config|
  config.vm.box = "debian/stretch64"

  current_hostname = "lobsters.xen.prgmr.com"

  config.vm.define current_hostname do |machine|
    machine.vm.hostname = current_hostname
    machine.vm.provision "ansible" do |ansible|

        ansible.groups = {
            "db" => current_hostname,
            "mx" => current_hostname,
            "smtp" => current_hostname,
            "www" => current_hostname,
            "www-worker" => current_hostname
          }

        ansible.compatibility_mode = "2.0"
        ansible.playbook = "site.yml"
    end
  end

end
