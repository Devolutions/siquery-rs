Import-Module "$PSScriptRoot/../siquery"

Describe 'wmi_os_version table' {
	InModuleScope siquery {
		Context 'schema validation' {
			BeforeAll {
				$SkipTableTest = (-Not $IsWindows) ? $true : $false
				$allSchemas = Get-ChildItem "$PSScriptRoot/../siquery/schema/*.json"
			}

			It 'has a schema' {
				$wmi_os_version = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_os_version' }
				$wmi_os_version | Should -Not -BeNullOrEmpty
			}

			# I want to nest these as dependent tests but am unsure on the right syntax.
			It 'has a valid schema' {
				$wmi_os_version = $allSchemas | Where-Object { $_.BaseName -eq 'wmi_os_version' }
				$schema = Get-Content $wmi_os_version.FullName -Raw
				$schema | Test-Json | Should -Be $true
			}

			# Windows Only Table -
			It 'validates schema' -Skip:$SkipTableTest {
				$wmi_os_version = Get-Siq 'wmi_os_version' | Select-Object -First 1

				$schema = Get-Content ($allSchemas | Where-Object { $_.BaseName -eq 'wmi_os_version' }).FullName -Raw

				$json = $wmi_os_version | ConvertTo-Json -ErrorAction Stop
				$json | Test-Json -Schema $schema | Should -Be $true
			}
		}
	}
}
