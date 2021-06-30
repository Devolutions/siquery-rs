Import-Module "$PSScriptRoot/../siquery"

Describe 'etc_hosts table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll { $allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json" }
			It 'has a schema' {
				$etc_hosts = $allSchemas | Where-Object { $_.BaseName -eq 'etc_hosts' }
				$etc_hosts | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$etc_hosts = $allSchemas | Where-Object { $_.BaseName -eq 'etc_hosts' }
				$schema = Get-Content $etc_hosts.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			It 'validates schema' {
				$etc_hosts = Get-Siq 'etc_hosts' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'etc_hosts' }).FullName -Raw

				$json = $etc_hosts | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
