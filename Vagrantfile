Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/xenial64"

  current_hostname = "lobsters.xen.prgmr.com"

  config.vm.define current_hostname do |machine|
    machine.vm.hostname = current_hostname

    # Ubuntu Xenial doesn't come with Python, which is needed for Ansible pipelining
    machine.vm.provision "shell" do |s|
      s.inline = "apt-get install -y python"
    end

    machine.vm.provision "ansible" do |ansible|

        ansible.groups = {
            "db" => current_hostname,
            "mx" => current_hostname,
            "smtp" => current_hostname,
            "www" => current_hostname,
            "www-worker" => current_hostname
          }

        ansible.compatibility_mode = "2.0"
        ansible.playbook = "prod.yml"
    end
  end

end
